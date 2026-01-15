use std::fs;

#[derive(Debug, Clone)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub pid: Option<u32>,
    pub process_name: String,
}

pub struct PortScanner;

impl PortScanner {
    pub fn scan_local_ports() -> Vec<PortInfo> {
        let mut results = Vec::new();

        // Linux ONLY
        if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
            for line in content.lines().skip(1) {
                // Skip header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() > 3 {
                    let local_addr = parts[1];
                    if let Some(port_hex) = local_addr.split(":").nth(1) {
                        if let Ok(port) = u16::from_str_radix(port_hex, 16) {
                            // let inode = parts[9]

                            results.push(PortInfo {
                                port,
                                protocol: "TCP".to_string(),
                                pid: None,
                                process_name: "Unknown".to_string(),
                            });
                        }
                    }
                }
            }
        }
        results
    }
}
