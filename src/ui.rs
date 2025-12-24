use crate::app::{App, InputMode};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Define layout: Title, Input Box, Error (optional), Logs, Notice, Keybinds
    let mut constraints = vec![
        Constraint::Length(3), // Title
        Constraint::Max(3),    // Input Box
    ];

    if app.error_message.is_some() {
        constraints.push(Constraint::Length(3)); // Error Message
    }

    constraints.extend(vec![
        Constraint::Min(3),    // Logs Widget
        Constraint::Length(1), // Notice Message
        Constraint::Length(1), // Keybinds Help
    ]);

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

    // Calculate indices for notice and keybinds (last two rows)
    let notice_index = chunks.len() - 2;
    let keybinds_index = chunks.len() - 1;

    // Render Logs Widget
    let logs_guard = app.logs.try_lock();
    if let Ok(logs) = logs_guard {
        let viewport_height = chunks[log_chunk_index].height.saturating_sub(2) as usize; // -2 for borders
        let logs_len = logs.len();

        // Store viewport height for scroll functions to use
        app.viewport_height = viewport_height;

        // Auto-scroll to bottom: position so the last log is at the bottom
        if app.auto_scroll && !logs.is_empty() {
            // Position so that the last item is visible at the bottom (with space for EOF)
            let visible_logs = viewport_height.saturating_sub(1); // -1 for EOF indicator
            app.scroll_offset = logs_len.saturating_sub(visible_logs).max(0);
            app.viewport_cursor = logs_len.saturating_sub(app.scroll_offset) - 1;
        }

        // Calculate scroll position: we want to show logs such that cursor can reach EOF at bottom
        // Total items shown: actual logs + EOF indicator = logs_len + 1
        // But viewport can only show viewport_height items

        let start_offset = app.scroll_offset;
        let end_offset = (start_offset + viewport_height).min(logs_len);

        // Calculate max cursor position: can be on any visible log OR on the EOF indicator
        let visible_log_count = end_offset.saturating_sub(start_offset);
        let max_cursor = if end_offset >= logs_len {
            // EOF indicator is visible, cursor can reach it
            visible_log_count // This includes the EOF indicator position
        } else {
            // EOF indicator is not visible
            visible_log_count.saturating_sub(1)
        };

        let cursor = app.viewport_cursor.min(max_cursor);

        // Create list items from visible range
        let mut log_items: Vec<ListItem> = logs[start_offset..end_offset]
            .iter()
            .map(|l| ListItem::new(format_log_line(l)))
            .collect();

        // Add EOF indicator if we've reached the end
        if end_offset >= logs_len && !logs.is_empty() {
            log_items.push(ListItem::new(Line::from(Span::styled(
                "--- END OF FILE ---",
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ))));
        }

        let log_list = List::new(log_items)
            .block(Block::default().title(" Logs ").borders(Borders::ALL))
            .highlight_style(Style::default().bg(Color::DarkGray)) // Visual cue for selection
            .highlight_symbol(">> ");

        // Set cursor position within the visible list (including EOF)
        let mut state = ListState::default();
        state.select(Some(cursor));

        frame.render_stateful_widget(log_list, chunks[log_chunk_index], &mut state);
    }

    // Render Notice Message (second line from bottom)
    let notice_text = get_notice_message(app);
    let notice_widget = Paragraph::new(notice_text)
        .style(Style::default().fg(Color::Cyan))
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(notice_widget, chunks[notice_index]);

    // Render Keybinds Help (last line)
    let keybinds_text = get_keybinds_help(app);
    let keybinds_widget = Paragraph::new(keybinds_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(keybinds_widget, chunks[keybinds_index]);
}

fn get_notice_message(_app: &App) -> String {
    // You can customize this to show different messages
    // For example: config loaded, file changed, etc.
    "✓ Configuration loaded successfully | Ready to tail logs".to_string()
}

fn get_keybinds_help(app: &App) -> String {
    format!(
        "[{}] quit | [{}] edit | [{}] scroll-up | [{}] scroll-down",
        app.config.keybinds.quit,
        app.config.keybinds.edit,
        app.config.keybinds.scroll_up,
        app.config.keybinds.scroll_down
    )
}

// TODO: Allow Configuration (.toml)
fn format_log_line(line: &str) -> Line<'_> {
    let (level, color) = if line.to_uppercase().contains("ERROR") || line.contains(" SEV ") {
        (" ERROR ", Color::Red)
    } else if line.to_uppercase().contains("WARN") {
        (" WARN  ", Color::Yellow)
    } else if line.to_uppercase().contains("INFO") {
        (" INFO  ", Color::Green)
    } else if line.to_uppercase().contains("DEBUG") {
        (" DEBUG ", Color::Blue)
    } else {
        ("", Color::White) // Default for lines without a clear level
    };

    if level.is_empty() {
        Line::from(Span::raw(line))
    } else {
        Line::from(vec![
            Span::styled(
                level,
                Style::default()
                    .bg(color)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(line, Style::default().fg(color)),
        ])
    }
}
