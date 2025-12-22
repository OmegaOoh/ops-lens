use crate::app::{App, InputMode};
use ratatui::{
    Frame, layout::{Constraint, Direction, Layout}, style::{Color, Style}, widgets::{Block, Borders, Gauge, List, ListItem, Paragraph}
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Define a simple layout: Top for title, Bottom for stats
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(3),    // Input Box
            Constraint::Min(3),    // Logs Widget
        ])
        .split(frame.area());

    // Title Widget
    let title = Paragraph::new("OpsLens: System & Cloud Observability")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Render Input Box
    let input_style = match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Edit => Style::default().fg(Color::Yellow),
    };

    let input_box = Paragraph::new(app.log_file_input.as_str())
            .style(input_style)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(format!(" Tail File (Press 'e' to edit, 'Esc' to cancel): {} ", app.current_path)));
        
    frame.render_widget(input_box, chunks[1]);

    // Render Logs Widget
    let logs_guard = app.logs.try_lock();
    let log_items: Vec<ListItem> = match &logs_guard {
        Ok(logs) => logs.iter().map(|l| ListItem::new(l.as_str())).collect(),
        Err(_) => vec![ListItem::new("Loading logs...")],
    };

    let log_list = List::new(log_items).block(
        Block::default()
            .title(" System Logs ")
            .borders(Borders::ALL),
    );

    frame.render_widget(log_list, chunks[2]);
}
