use crate::app::App;
use crate::tabs::TabState;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Tabs},
};

pub fn render(app: &mut App, frame: &mut Frame) {
    // Define layout with tabs
    let mut constraints = vec![
        Constraint::Length(3), // 0: Title
        Constraint::Length(2), // 1: Tabs
    ];

    let log_level_top_enabled =
        app.config.log_level.enabled && app.config.log_level.indicator_position == "top";
    let log_level_bottom_enabled =
        app.config.log_level.enabled && app.config.log_level.indicator_position == "bottom";

    // Add log level indicator if enabled and position is "top"
    if log_level_top_enabled {
        constraints.push(Constraint::Length(1)); // 2: Log Level Indicator (top)
    }

    let content_chunk_index = constraints.len();
    constraints.push(Constraint::Min(3)); // Content Widget (depends on tab)

    // Add log level indicator if enabled and position is "bottom"
    if log_level_bottom_enabled {
        constraints.push(Constraint::Length(1)); // Log Level Indicator (bottom)
    }

    let error_chunk_index = constraints.len();
    constraints.extend(vec![
        Constraint::Length(1), // Error/Status Message (second line from bottom)
        Constraint::Length(1), // Keybinds Help (last line)
    ]);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(frame.area());

    // Title Widget
    let title = Paragraph::new("OpsLens: System & Cloud Observability")
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(title, chunks[0]);

    // Render Tabs using TabManager
    let tab_names = app.tab_manager.tab_names();
    let selected_tab = app.tab_manager.current_index();
    let tabs_widget = Tabs::new(tab_names)
        .select(selected_tab)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(tabs_widget, chunks[1]);

    // Render tab-specific content using the state pattern
    render_current_tab(app, frame, chunks[content_chunk_index]);

    // Render bottom log level indicator if enabled
    if log_level_bottom_enabled {
        let indicator_text = get_log_level_indicator(app);
        let indicator_widget = Paragraph::new(indicator_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(indicator_widget, chunks[error_chunk_index - 1]);
    }

    // Render Error/Status Message (second line from bottom)
    let error_text = if let Some(error_msg) = &app.error_message {
        format!("❌ {}", error_msg)
    } else {
        String::new()
    };
    let error_widget = Paragraph::new(error_text)
        .style(Style::default().fg(Color::Red))
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(error_widget, chunks[error_chunk_index]);

    // Render Keybinds Help (last line)
    let keybinds_text = get_keybinds_help(app);
    let keybinds_widget = Paragraph::new(keybinds_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(keybinds_widget, chunks[error_chunk_index + 1]);
}

/// Render the current tab's content
/// This is a helper function to handle the borrowing correctly
fn render_current_tab(app: &mut App, frame: &mut Frame, area: ratatui::prelude::Rect) {
    let current_type = app.tab_manager.current_type();

    match current_type {
        crate::tabs::TabType::LogReader => {
            let mut state = crate::tabs::LogReaderTab::new();
            state.render(app, frame, area);
        }
        crate::tabs::TabType::Settings => {
            let mut state = crate::tabs::SettingsTab::new();
            state.render(app, frame, area);
        }
    }
}

fn get_keybinds_help(app: &App) -> String {
    format!(
        "[{}] quit | [{}] edit | [{}] scroll-up | [{}] scroll-down | [Tab] switch tabs",
        app.config.keybinds.quit,
        app.config.keybinds.edit,
        app.config.keybinds.scroll_up,
        app.config.keybinds.scroll_down
    )
}

fn get_log_level_indicator(app: &App) -> String {
    let logs = match app.logs.try_lock() {
        Ok(logs) => logs,
        Err(_) => return String::new(),
    };

    if !app.config.log_level.show_level_counts {
        return String::new();
    }

    let mut error_count = 0;
    let mut warning_count = 0;
    let mut info_count = 0;
    let mut debug_count = 0;

    // Count log levels
    for line in logs.iter() {
        let line_upper = line.to_uppercase();
        let mut counted = false;

        // Check error patterns
        for pattern in &app.config.log_level.levels.error.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                error_count += 1;
                counted = true;
                break;
            }
        }

        if counted {
            continue;
        }

        // Check warning patterns
        for pattern in &app.config.log_level.levels.warning.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                warning_count += 1;
                counted = true;
                break;
            }
        }

        if counted {
            continue;
        }

        // Check info patterns
        for pattern in &app.config.log_level.levels.info.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                info_count += 1;
                counted = true;
                break;
            }
        }

        if counted {
            continue;
        }

        // Check debug patterns
        for pattern in &app.config.log_level.levels.debug.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                debug_count += 1;
                break;
            }
        }
    }

    format!(
        "Levels: {} ERROR | {} WARNING | {} INFO | {} DEBUG",
        error_count, warning_count, info_count, debug_count
    )
}
