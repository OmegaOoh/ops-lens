pub mod linux;

use crate::port::PortInfo;

pub trait ScannerStrategy {
    fn scan_ports(&self) -> Vec<PortInfo>;
    fn get_process_name(&self, pid: u32) -> String;
}
