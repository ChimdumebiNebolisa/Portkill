use std::collections::HashMap;
use std::process::Command;

use crate::error::{command_not_found, PortkillError, Result};
use crate::platform::{Platform, ProcessInfo};

pub struct UnixPlatform;

impl UnixPlatform {
    fn lsof_pids(&self, port: u16) -> Result<Vec<u32>> {
        let mut cmd = Command::new("lsof");
        cmd.args(["-i", &format!(":{}", port), "-t"]);
        let output = match cmd.output() {
            Ok(o) => o,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(command_not_found(
                    "lsof",
                    "Install lsof (e.g. on Debian/Ubuntu: apt install lsof; on macOS it is usually pre-installed).",
                ))
            }
            Err(e) => {
                return Err(PortkillError::CommandFailed {
                    command: "lsof -i :PORT -t".to_string(),
                    source: e,
                })
            }
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let pids: Vec<u32> = stdout
            .lines()
            .filter_map(|line| line.trim().parse::<u32>().ok())
            .filter(|&p| p > 0)
            .collect();
        Ok(pids)
    }

    fn get_process_names(&self, pids: &[u32]) -> HashMap<u32, String> {
        if pids.is_empty() {
            return HashMap::new();
        }
        let pid_list: Vec<String> = pids.iter().map(|p| p.to_string()).collect();
        let output = match Command::new("ps")
            .args(["-o", "pid=,comm=", "-p", &pid_list.join(",")])
            .output()
        {
            Ok(o) => o,
            Err(_) => return HashMap::new(),
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut map = HashMap::new();
        for line in stdout.lines() {
            let line = line.trim();
            let mut parts = line.splitn(2, char::is_whitespace);
            if let (Some(pid_str), Some(name)) = (parts.next(), parts.next()) {
                if let Ok(pid) = pid_str.trim().parse::<u32>() {
                    map.insert(pid, name.trim().to_string());
                }
            }
        }
        map
    }
}

impl Platform for UnixPlatform {
    fn find_processes_on_port(&self, port: u16) -> Result<Vec<ProcessInfo>> {
        let pids = self.lsof_pids(port)?;
        if pids.is_empty() {
            return Err(PortkillError::NoProcessOnPort(port));
        }
        let mut unique: std::collections::HashSet<u32> = pids.into_iter().collect();
        let pids: Vec<u32> = unique.into_iter().collect();
        let names = self.get_process_names(&pids);
        let processes = pids
            .into_iter()
            .map(|pid| ProcessInfo {
                pid,
                name: names.get(&pid).cloned(),
            })
            .collect();
        Ok(processes)
    }

    fn kill_process(&self, pid: u32, force: bool) -> Result<()> {
        let sig = if force { "-9" } else { "-15" };
        let status = Command::new("kill")
            .args([sig, &pid.to_string()])
            .status()
            .map_err(|e| PortkillError::CommandFailed {
                command: format!("kill {} {}", sig, pid),
                source: e,
            })?;
        if status.success() {
            Ok(())
        } else {
            let code = status.code().unwrap_or(-1);
            Err(PortkillError::KillFailed {
                pid,
                message: format!("kill exited with code {}", code),
            })
        }
    }
}
