use std::sync::Arc;

use tokio::sync::Mutex;

pub enum InputMode {
    Normal,
    Edit,
}

pub struct App {
    pub running: bool,
    pub log_file_input: String,
    pub input_mode: InputMode,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub current_path: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            log_file_input: String::new(),
            input_mode: InputMode::Normal,
            logs: Arc::new(Mutex::new(Vec::new())),
            current_path: String::from("No file selected"),
        }
    }
    pub fn quit(&mut self) {
        self.running = false;
    }
}
