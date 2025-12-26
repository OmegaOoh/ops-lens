use crate::app::{App, InputMode, Tab};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Tabs},
};

fn string_to_color(color_str: &str) -> Color {
    match color_str.to_lowercase().as_str() {
        "red" => Color::Red,
        "yellow" => Color::Yellow,
        "green" => Color::Green,
        "blue" => Color::Blue,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "white" => Color::White,
        "gray" | "grey" => Color::Gray,
        "dark_gray" | "dark_grey" => Color::DarkGray,
        _ => Color::White,
    }
}

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

    // Render Tabs
    let tabs = vec!["Log Reader", "Settings"];
    let selected_tab = match app.current_tab {
        Tab::LogReader => 0,
        Tab::SimpleTab => 1,
    };
    let tabs_widget = Tabs::new(tabs)
        .select(selected_tab)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(tabs_widget, chunks[1]);

    // Render top log level indicator if enabled
    if log_level_top_enabled {
        let indicator_text = get_log_level_indicator(app);
        let indicator_widget = Paragraph::new(indicator_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(indicator_widget, chunks[2]);
    }

    // Render tab-specific content
    match app.current_tab {
        Tab::LogReader => render_log_reader_tab(app, frame, chunks[content_chunk_index]),
        Tab::SimpleTab => render_settings_tab(app, frame, chunks[content_chunk_index]),
    }

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

fn render_log_reader_tab(app: &mut App, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
    // Define layout for log reader tab
    let mut constraints = vec![
        Constraint::Max(3), // Input Box
    ];

    let log_level_top_enabled =
        app.config.log_level.enabled && app.config.log_level.indicator_position == "top";

    // Add log level indicator if enabled and position is "top"
    if log_level_top_enabled {
        constraints.push(Constraint::Length(1)); // Log Level Indicator (top)
    }

    let logs_chunk_index = constraints.len();
    constraints.push(Constraint::Min(3)); // Logs Widget

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);

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

    frame.render_widget(input_box, chunks[0]);

    // Render top log level indicator if enabled
    if log_level_top_enabled {
        let indicator_text = get_log_level_indicator(app);
        let indicator_widget = Paragraph::new(indicator_text)
            .style(Style::default().fg(Color::Cyan))
            .alignment(ratatui::layout::Alignment::Left);
        frame.render_widget(indicator_widget, chunks[1]);
    }

    // Render Logs Widget
    let logs_guard = app.logs.try_lock();
    if let Ok(logs) = logs_guard {
        let viewport_height = chunks[logs_chunk_index].height.saturating_sub(2) as usize; // -2 for borders
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
            .map(|l| ListItem::new(format_log_line(l, app)))
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

        frame.render_stateful_widget(log_list, chunks[logs_chunk_index], &mut state);
    }
}

fn render_settings_tab(app: &App, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
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

fn format_log_line(line: &str, app: &App) -> Line<'static> {
    let line_upper = line.to_uppercase();
    let line_owned = line.to_string();
    let mut matched_level: Option<(String, Color)> = None;

    // Check error patterns
    for pattern in &app.config.log_level.levels.error.patterns {
        if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
            let color = string_to_color(&app.config.log_level.levels.error.color);
            matched_level = Some((
                app.config.log_level.levels.error.display_name.clone(),
                color,
            ));
            break;
        }
    }

    // Check warning patterns
    if matched_level.is_none() {
        for pattern in &app.config.log_level.levels.warning.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                let color = string_to_color(&app.config.log_level.levels.warning.color);
                matched_level = Some((
                    app.config.log_level.levels.warning.display_name.clone(),
                    color,
                ));
                break;
            }
        }
    }

    // Check info patterns
    if matched_level.is_none() {
        for pattern in &app.config.log_level.levels.info.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                let color = string_to_color(&app.config.log_level.levels.info.color);
                matched_level =
                    Some((app.config.log_level.levels.info.display_name.clone(), color));
                break;
            }
        }
    }

    // Check debug patterns
    if matched_level.is_none() {
        for pattern in &app.config.log_level.levels.debug.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                let color = string_to_color(&app.config.log_level.levels.debug.color);
                matched_level = Some((
                    app.config.log_level.levels.debug.display_name.clone(),
                    color,
                ));
                break;
            }
        }
    }

    if let Some((display_name, color)) = matched_level {
        Line::from(vec![
            Span::styled(
                display_name,
                Style::default()
                    .bg(color)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(line_owned, Style::default().fg(color)),
        ])
    } else {
        Line::from(Span::raw(line_owned))
    }
}
