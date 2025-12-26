mod log_reader;
mod settings;
mod tab_state;

pub use log_reader::LogReaderTab;
pub use settings::SettingsTab;
pub use tab_state::TabState;

/// Represents the different tab types in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabType {
    LogReader,
    Settings,
}

impl TabType {
    /// Create a new tab state instance for this tab type
    pub fn create_state(&self) -> Box<dyn TabState> {
        match self {
            TabType::LogReader => Box::new(LogReaderTab::new()),
            TabType::Settings => Box::new(SettingsTab::new()),
        }
    }

    /// Get the display name for this tab
    pub fn display_name(&self) -> &'static str {
        match self {
            TabType::LogReader => "Log Reader",
            TabType::Settings => "Settings",
        }
    }

    /// Get the next tab in order
    pub fn next(&self) -> Self {
        match self {
            TabType::LogReader => TabType::Settings,
            TabType::Settings => TabType::LogReader,
        }
    }
}

/// Tab Manager - handles transitions between tab states
pub struct TabManager {
    current_tab_type: TabType,
    current_state: Box<dyn TabState>,
}

impl TabManager {
    /// Create a new TabManager starting with the LogReader tab
    pub fn new() -> Self {
        let current_tab_type = TabType::LogReader;
        let current_state = current_tab_type.create_state();
        Self {
            current_tab_type,
            current_state,
        }
    }

    /// Get the current tab type
    pub fn current_type(&self) -> TabType {
        self.current_tab_type
    }

    /// Get the current tab state
    #[allow(dead_code)]
    pub fn current_state(&self) -> &dyn TabState {
        &*self.current_state
    }

    /// Get mutable reference to the current tab state
    #[allow(dead_code)]
    pub fn current_state_mut(&mut self) -> &mut dyn TabState {
        &mut *self.current_state
    }

    /// Switch to the next tab
    pub fn next_tab(&mut self) {
        self.current_tab_type = self.current_tab_type.next();
        self.current_state = self.current_tab_type.create_state();
    }

    /// Switch to a specific tab type
    #[allow(dead_code)]
    pub fn switch_to(&mut self, tab_type: TabType) {
        if self.current_tab_type != tab_type {
            self.current_tab_type = tab_type;
            self.current_state = self.current_tab_type.create_state();
        }
    }

    /// Get the index of the current tab (for rendering)
    pub fn current_index(&self) -> usize {
        match self.current_tab_type {
            TabType::LogReader => 0,
            TabType::Settings => 1,
        }
    }

    /// Get all available tab names
    pub fn tab_names(&self) -> Vec<&'static str> {
        vec![
            TabType::LogReader.display_name(),
            TabType::Settings.display_name(),
        ]
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
