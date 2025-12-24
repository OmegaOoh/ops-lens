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
    pub scroll_offset: usize,
    pub viewport_cursor: usize, // Position of cursor within the viewport (0 to viewport_height-1)
    pub viewport_height: usize, // Track the current viewport height
    pub auto_scroll: bool,
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
            scroll_offset: 0,
            viewport_cursor: 0,
            viewport_height: 0,
            auto_scroll: true,
            error_message: None,
        }
    }

    pub fn reset_scroll(&mut self) {
        self.scroll_offset = 0;
        self.viewport_cursor = 0;
        self.scroll_state.select(None);
        self.auto_scroll = true;
    }

    pub async fn scroll_to_bottom(&mut self) {
        let logs = self.logs.lock().await;
        if !logs.is_empty() {
            let last_index = logs.len().saturating_sub(1);
            self.scroll_offset = last_index;
            self.scroll_state.select(Some(last_index));
        }
    }

    pub async fn auto_scroll_to_bottom(&mut self) {
        let logs = self.logs.lock().await;
        let last_index = logs.len().saturating_sub(1);
        self.scroll_offset = last_index;
        self.scroll_state.select(Some(last_index));
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

    pub fn clamp_cursor(&mut self, logs_len: usize, viewport_height: usize) {
        // Allow cursor to move until EOF indicator is at the bottom of the viewport
        if logs_len == 0 {
            self.viewport_cursor = 0;
            return;
        }

        let remaining_logs = logs_len.saturating_sub(self.scroll_offset);
        let max_cursor = if remaining_logs > 0 {
            remaining_logs.min(viewport_height - 1)
        } else {
            0
        };

        self.viewport_cursor = self.viewport_cursor.min(max_cursor);
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
