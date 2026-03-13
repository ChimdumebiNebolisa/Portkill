use std::collections::HashMap;

use crate::error::Result;

#[cfg(windows)]
mod windows;
#[cfg(unix)]
mod unix;

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: Option<String>,
}

pub trait Platform {
    fn find_processes_on_port(&self, port: u16) -> Result<Vec<ProcessInfo>>;
    fn kill_process(&self, pid: u32, force: bool) -> Result<()>;
}

pub fn current_platform() -> Box<dyn Platform> {
    #[cfg(windows)]
    return Box::new(super::windows::WindowsPlatform);

    #[cfg(unix)]
    return Box::new(super::unix::UnixPlatform);
}

pub fn get_unique_processes(processes: Vec<ProcessInfo>) -> Vec<ProcessInfo> {
    let mut seen: HashMap<u32, ProcessInfo> = HashMap::new();
    for p in processes {
        seen.entry(p.pid).or_insert(p);
    }
    let mut out: Vec<_> = seen.into_values().collect();
    out.sort_by_key(|p| p.pid);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_unique_processes_dedupes_and_sorts() {
        let processes = vec![
            ProcessInfo {
                pid: 42,
                name: Some("a".to_string()),
            },
            ProcessInfo {
                pid: 10,
                name: Some("b".to_string()),
            },
            ProcessInfo {
                pid: 42,
                name: Some("dupe".to_string()),
            },
        ];
        let out = get_unique_processes(processes);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].pid, 10);
        assert_eq!(out[1].pid, 42);
    }
}
