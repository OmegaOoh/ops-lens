use std::{
    collections::{HashMap, HashSet},
    fs,
};

use super::ScannerStrategy;
use crate::port::PortInfo;

pub struct LinuxScanner;

impl LinuxScanner {
    fn map_inodes_to_pids(&self, inodes: &HashSet<String>) -> HashMap<String, u32> {
        let mut map = HashMap::new();
        if inodes.is_empty() {
            return map;
        }

        if let Ok(proc_entries) = fs::read_dir("/proc") {
            for entry in proc_entries.flatten() {
                let file_name = entry.file_name();
                let pid_str = match file_name.to_str() {
                    Some(s) => s,
                    None => continue,
                };
                let pid: u32 = match pid_str.parse() {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                let fd_dir = format!("/proc/{}/fd", pid);
                let fd_iter = match fs::read_dir(&fd_dir) {
                    Ok(iter) => iter,
                    Err(_) => continue, // permission denied or process exited
                };

                for fd in fd_iter.flatten() {
                    let link_target = match fs::read_link(fd.path()) {
                        Ok(t) => t,
                        Err(_) => continue,
                    };
                    let target_str = link_target.to_string_lossy();

                    // symlink targets for sockets look like "socket:[12345]"
                    if let Some(start) = target_str.find("socket:[") {
                        if let Some(end) = target_str[start..].find(']') {
                            let inode = &target_str[start + 8..start + end]; // between brackets
                            if inodes.contains(inode) && !map.contains_key(inode) {
                                map.insert(inode.to_string(), pid);
                                if map.len() == inodes.len() {
                                    return map;
                                }
                            }
                        }
                    }
                }
            }
        }

        map
    }

    fn parse_tcp_table(
        &self,
        path: &str,
        proto: &'static str,
        out: &mut Vec<(u16, String, &'static str)>,
    ) {
        if let Ok(content) = fs::read_to_string(path) {
            for line in content.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();

                // Layout (whitespace-split):
                // 0 sl
                // 1 local_address
                // 2 rem_address
                // 3 st
                // 4 tx_queue:rx_queue
                // 5 tr:tm->when
                // 6 retrnsmt
                // 7 uid
                // 8 timeout
                // 9 inode
                // 10 ref
                // 11 pointer
                // 12 drops
                if parts.len() > 9 {
                    let local_addr = parts[1];
                    if let Some(port_hex) = local_addr.split(':').nth(1) {
                        if let Ok(port) = u16::from_str_radix(port_hex, 16) {
                            let inode = parts[9].to_string(); // correct inode column
                            if !inode.is_empty() && inode != "0" {
                                out.push((port, inode, proto));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl ScannerStrategy for LinuxScanner {
    fn scan_ports(&self) -> Vec<PortInfo> {
        // Collect (port, inode, proto) from /proc/net/tcp and /proc/net/tcp6
        let mut entries: Vec<(u16, String, &'static str)> = Vec::new();
        self.parse_tcp_table("/proc/net/tcp", "TCP", &mut entries);
        self.parse_tcp_table("/proc/net/tcp6", "TCP6", &mut entries);

        // Map inode -> pid
        let inode_set: HashSet<String> =
            entries.iter().map(|(_, inode, _)| inode.clone()).collect();
        let inode_pid = self.map_inodes_to_pids(&inode_set);

        // Build results
        let mut results = Vec::new();
        for (port, inode, proto) in entries {
            let pid_opt = inode_pid.get(&inode).copied();
            let process_name = match pid_opt {
                Some(pid) => self.get_process_name(pid),
                None => "Unknown".to_string(),
            };

            results.push(PortInfo {
                port,
                protocol: proto.to_string(),
                pid: pid_opt,
                process_name,
            });
        }

        results
    }

    fn get_process_name(&self, pid: u32) -> String {
        // Try /proc/<pid>/comm first
        let comm_path = format!("/proc/{}/comm", pid);
        if let Ok(name) = fs::read_to_string(&comm_path) {
            let n = name.trim();
            if !n.is_empty() {
                return n.to_string();
            }
        }

        // Fallback to /proc/<pid>/cmdline (NUL-separated)
        let cmdline_path = format!("/proc/{}/cmdline", pid);
        if let Ok(data) = fs::read(cmdline_path) {
            if !data.is_empty() {
                let parts: Vec<&str> = data
                    .split(|&b| b == 0)
                    .filter_map(|s| std::str::from_utf8(s).ok())
                    .filter(|s| !s.is_empty())
                    .collect();
                if let Some(first) = parts.first() {
                    if let Some(name) = first.rsplit('/').next() {
                        return name.to_string();
                    } else {
                        return first.to_string();
                    }
                }
            }
        }

        "Unknown".to_string()
    }
}
