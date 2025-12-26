use crate::app::App;
use ratatui::prelude::Rect;

/// Trait that defines the interface for all tab states
/// Each tab implements this trait to define its own rendering and behavior
pub trait TabState: Send + Sync {
    /// Render the tab content to the frame
    /// The app reference is mutable to allow state updates during rendering
    fn render(&mut self, app: &mut App, frame: &mut ratatui::Frame, area: Rect);

    /// Get the display name of this tab
    #[allow(dead_code)]
    fn name(&self) -> &str;

    /// Optional: Called when entering this tab state
    #[allow(dead_code)]
    fn on_enter(&mut self, _app: &mut App) {}

    /// Optional: Called when leaving this tab state
    #[allow(dead_code)]
    fn on_exit(&mut self, _app: &mut App) {}

    /// Optional: Get a description or help text for this tab
    #[allow(dead_code)]
    fn help_text(&self) -> Option<&str> {
        None
    }
}

/// Context passed to tab state methods for rendering
#[allow(dead_code)]
pub struct TabStateContext {
    pub area: Rect,
}
