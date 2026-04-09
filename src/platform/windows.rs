use std::collections::HashMap;
use std::process::Command;

use crate::error::{check_command_output, PortkillError, Result};
use crate::platform::{Platform, ProcessInfo};

pub struct WindowsPlatform;

impl WindowsPlatform {
    fn local_endpoint_matches_port(local: &str, port: u16) -> bool {
        local
            .rsplit_once(':')
            .map(|(_, local_port)| local_port == port.to_string())
            .unwrap_or(false)
    }

    fn netstat_pids(&self, port: u16) -> Result<Vec<u32>> {
        let output = check_command_output(Command::new("netstat").args(["-ano"]), "netstat -ano")?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut pids = Vec::new();
        for line in stdout.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("Proto") || line.starts_with("Active") {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }
            let local = parts.get(1).copied().unwrap_or("");
            if !Self::local_endpoint_matches_port(local, port) {
                continue;
            }
            if let Some(last) = parts.last() {
                if let Ok(pid) = last.parse::<u32>() {
                    if pid > 0 {
                        pids.push(pid);
                    }
                }
            }
        }
        Ok(pids)
    }

    fn tasklist_names(&self) -> Result<HashMap<u32, String>> {
        let output = check_command_output(
            Command::new("tasklist").args(["/FO", "CSV", "/NH"]),
            "tasklist",
        )?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut map = HashMap::new();
        for line in stdout.lines() {
            let line = line.trim();
            let parts: Vec<&str> = line.split("\",\"").collect();
            if parts.len() < 2 {
                continue;
            }
            let name = parts[0].trim_matches('"').to_string();
            let pid_str: String = parts[1]
                .chars()
                .take_while(|c| c.is_ascii_digit())
                .collect();
            if let Ok(pid) = pid_str.parse::<u32>() {
                map.insert(pid, name);
            }
        }
        Ok(map)
    }
}

impl Platform for WindowsPlatform {
    fn find_processes_on_port(&self, port: u16) -> Result<Vec<ProcessInfo>> {
        let pids = self.netstat_pids(port)?;
        if pids.is_empty() {
            return Err(PortkillError::NoProcessOnPort(port));
        }
        let unique: std::collections::HashSet<u32> = pids.into_iter().collect();
        let pids: Vec<u32> = unique.into_iter().collect();
        let names = self.tasklist_names().unwrap_or_default();
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
        let mut cmd = Command::new("taskkill");
        cmd.args(["/PID", &pid.to_string()]);
        if force {
            cmd.arg("/F");
        }
        let status = cmd.status().map_err(|e| PortkillError::CommandFailed {
            command: format!("taskkill /PID {}", pid),
            source: e,
        })?;
        if status.success() {
            Ok(())
        } else {
            let code = status.code().unwrap_or(-1);
            Err(PortkillError::KillFailed {
                pid,
                message: format!("taskkill exited with code {}", code),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WindowsPlatform;

    #[test]
    fn local_endpoint_matches_port_requires_exact_match() {
        assert!(WindowsPlatform::local_endpoint_matches_port(
            "0.0.0.0:3000",
            3000
        ));
        assert!(WindowsPlatform::local_endpoint_matches_port(
            "[::]:3000",
            3000
        ));
        assert!(WindowsPlatform::local_endpoint_matches_port(
            "[fe80::1%12]:3000",
            3000
        ));
        assert!(!WindowsPlatform::local_endpoint_matches_port(
            "0.0.0.0:3000",
            300
        ));
        assert!(!WindowsPlatform::local_endpoint_matches_port(
            "[::]:3000",
            30
        ));
        assert!(!WindowsPlatform::local_endpoint_matches_port("*:*", 3000));
    }
}
