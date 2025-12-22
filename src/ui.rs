use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};
use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
    // Define a simple layout: Top for title, Bottom for stats
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // Main Content
        ])
        .split(frame.area());

    // Title Widget
    let title = Paragraph::new("OpsLens: System & Cloud Observability")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // A Gauge widget to show our "CPU" state
    let gauge = Gauge::default()
        .block(Block::default().title(" CPU Usage ").borders(Borders::ALL))
        .gauge_style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
        .percent(app.cpu_usage as u16);
    
    frame.render_widget(gauge, chunks[1]);
}