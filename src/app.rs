use std::sync::Arc;

use ratatui::widgets::ListState;
use tokio::sync::Mutex;

use crate::config::Config;

pub enum InputMode {
    Normal,
    Edit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    LogReader,
    SimpleTab,
}

pub struct App {
    pub running: bool,
    pub log_file_input: String,
    pub input_mode: InputMode,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub current_path: String,
    pub scroll_state: ListState,
    pub scroll_offset: usize,
    pub viewport_cursor: usize, // Position of cursor within the viewport (0 to viewport_height-1)
    pub viewport_height: usize, // Track the current viewport height
    pub auto_scroll: bool,
    pub error_message: Option<String>,
    pub config: Config,
    pub current_tab: Tab,
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
            scroll_offset: 0,
            viewport_cursor: 0,
            viewport_height: 0,
            auto_scroll: true,
            error_message: None,
            config: Config::load_or_create("ops-lens.toml"),
            current_tab: Tab::LogReader,
        }
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
        self.viewport_cursor = 0;
        self.scroll_state.select(None);
        self.auto_scroll = true;
    }

    pub async fn scroll_down(&mut self) {
        let logs = self.logs.lock().await;
        let logs_len = logs.len();

        if logs_len == 0 {
            return;
        }

        self.auto_scroll = false;

        let absolute_position = self.scroll_offset + self.viewport_cursor;

        // Allow scrolling until EOF indicator is at the bottom
        // EOF is at position logs_len, so we can go one position beyond the last log
        if absolute_position >= logs_len {
            return;
        }

        // Move cursor or scroll offset
        let viewport_height = self.viewport_height;
        if viewport_height > 0 {
            // If cursor is not at the bottom of viewport, move it down
            if self.viewport_cursor < viewport_height - 1 {
                self.viewport_cursor += 1;
            } else {
                // Cursor is at bottom of viewport, scroll the offset
                self.scroll_offset += 1;
            }
        } else {
            // Fallback if viewport_height is not set
            self.viewport_cursor += 1;
        }
    }

    pub async fn scroll_up(&mut self) {
        self.auto_scroll = false;

        // If cursor is not at the top of viewport, move cursor up
        if self.viewport_cursor > 0 {
            self.viewport_cursor -= 1;
        } else if self.scroll_offset > 0 {
            // Cursor is at top of viewport, scroll the offset up
            self.scroll_offset -= 1;
        }
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

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::LogReader => Tab::SimpleTab,
            Tab::SimpleTab => Tab::LogReader,
        };
    }
}
