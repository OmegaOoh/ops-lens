use crate::strategy::scanner::{ScannerStrategy, linux::LinuxScanner};

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
        let strategy: Box<dyn ScannerStrategy> = if cfg!(target_os = "linux") {
            Box::new(LinuxScanner)
        } else {
            panic!("Unsupported operating system");
        };

        strategy.scan_ports()
    }
}
