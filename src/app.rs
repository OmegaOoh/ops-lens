use std::sync::Arc;

use ratatui::widgets::ListState;
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
    pub scroll_state: ListState,
    pub error_message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            log_file_input: String::new(),
            input_mode: InputMode::Normal,
            logs: Arc::new(Mutex::new(Vec::new())),
            current_path: String::from("No file selected"),
            scroll_state: ListState::default(),
            error_message: None,
        }
    }

    pub fn scroll_down(&mut self) {
        let i = match self.scroll_state.selected() {
            Some(i) => i + 1,
            None => 0,
        };
        self.scroll_state.select(Some(i));
    }

    pub fn scroll_up(&mut self) {
        let i = match self.scroll_state.selected() {
            Some(i) => {
                if i > 0 {
                    i - 1
                } else {
                    0
                }
            }
            None => 0,
        };
        self.scroll_state.select(Some(i));
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}
