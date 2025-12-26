use super::TabState;
use crate::app::App;
use ratatui::{
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// SettingsTab - The state for the settings tab
pub struct SettingsTab;

impl SettingsTab {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SettingsTab {
    fn default() -> Self {
        Self::new()
    }
}

impl TabState for SettingsTab {
    fn name(&self) -> &'static str {
        "Settings"
    }

    fn render(&mut self, app: &mut App, frame: &mut ratatui::Frame, area: Rect) {
        let settings_content = vec![
            Line::from(Span::styled(
                "Settings",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from("Configuration Options:"),
            Line::from(""),
            Line::from(Span::raw("• Log Level Detection: ")),
            if app.config.log_level.enabled {
                Line::from(Span::styled("  Enabled", Style::default().fg(Color::Green)))
            } else {
                Line::from(Span::styled("  Disabled", Style::default().fg(Color::Red)))
            },
            Line::from(""),
            Line::from(Span::raw("• Log Level Indicator Position: ")),
            Line::from(format!("  {}", app.config.log_level.indicator_position)),
            Line::from(""),
            Line::from(Span::raw("• Show Level Counts: ")),
            if app.config.log_level.show_level_counts {
                Line::from(Span::styled("  Enabled", Style::default().fg(Color::Green)))
            } else {
                Line::from(Span::styled("  Disabled", Style::default().fg(Color::Red)))
            },
            Line::from(""),
            Line::from("Edit the ops-lens.toml file to modify settings."),
        ];

        let settings_widget = Paragraph::new(settings_content)
            .block(Block::default().title(" Settings ").borders(Borders::ALL))
            .style(Style::default().fg(Color::White));

        frame.render_widget(settings_widget, area);
    }
}
