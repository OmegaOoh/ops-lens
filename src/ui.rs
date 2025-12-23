use crate::app::{App, InputMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Define layout based on whether there's an error
    let constraints = if app.error_message.is_some() {
        vec![
            Constraint::Length(3), // Title
            Constraint::Max(3),    // Input Box
            Constraint::Length(3), // Error Message
            Constraint::Min(1),    // Logs Widget
        ]
    } else {
        vec![
            Constraint::Length(3), // Title
            Constraint::Max(3),    // Input Box
            Constraint::Min(3),    // Logs Widget
        ]
    };

    // Define a simple layout: Top for title, Bottom for stats
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
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
        .block(Block::default().borders(Borders::ALL).title(format!(
            " Tail File (Press 'e' to edit, 'Esc' to cancel): {} ",
            app.current_path
        )));

    frame.render_widget(input_box, chunks[1]);

    // Render Error Message if present
    let mut log_chunk_index = 2;
    if let Some(error_msg) = &app.error_message {
        let error_widget = Paragraph::new(error_msg.as_str())
            .style(Style::default().fg(Color::Red))
            .block(Block::default().borders(Borders::ALL).title(" Error "));
        frame.render_widget(error_widget, chunks[2]);
        log_chunk_index = 3;
    }

    // Render Logs Widget
    let logs_guard = app.logs.try_lock();
    if let Ok(logs) = logs_guard {
        let log_items: Vec<ListItem> = logs.iter().map(|l| ListItem::new(l.as_str())).collect();

        let log_list = List::new(log_items)
            .block(Block::default().title(" Logs ").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray)) // Visual cue for selection
            .highlight_symbol(">> ");

        frame.render_stateful_widget(log_list, chunks[log_chunk_index], &mut app.scroll_state);
    }
}
