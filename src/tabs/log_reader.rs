use crate::app::App;
use crate::tabs::TabState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

/// LogReaderTab - The state for the log reader tab
pub struct LogReaderTab {
    #[allow(dead_code)]
    list_state: ListState,
}

impl LogReaderTab {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

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

    fn format_log_line(line: &str, app: &App) -> Line<'static> {
        let line_upper = line.to_uppercase();
        let line_owned = line.to_string();
        let mut matched_level: Option<(String, Color)> = None;

        // Check error patterns
        for pattern in &app.config.log_level.levels.error.patterns {
            if line_upper.contains(&pattern.to_uppercase()) || line.contains(pattern) {
                let color = Self::string_to_color(&app.config.log_level.levels.error.color);
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
                    let color = Self::string_to_color(&app.config.log_level.levels.warning.color);
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
                    let color = Self::string_to_color(&app.config.log_level.levels.info.color);
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
                    let color = Self::string_to_color(&app.config.log_level.levels.debug.color);
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
}

impl Default for LogReaderTab {
    fn default() -> Self {
        Self::new()
    }
}

impl TabState for LogReaderTab {
    fn name(&self) -> &'static str {
        "Log Reader"
    }

    fn render(&mut self, app: &mut App, frame: &mut ratatui::Frame, area: Rect) {
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
            crate::app::InputMode::Normal => Style::default(),
            crate::app::InputMode::Edit => Style::default().fg(Color::Yellow),
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
            let indicator_text = Self::get_log_level_indicator(app);
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

            // Auto-scroll to bottom: position so the last log is at the bottom
            if app.auto_scroll && !logs.is_empty() {
                // Position so that the last item is visible at the bottom (with space for EOF)
                let visible_logs = viewport_height.saturating_sub(1); // -1 for EOF indicator
                app.scroll_offset = logs_len.saturating_sub(visible_logs).max(0);
                app.viewport_cursor = logs_len.saturating_sub(app.scroll_offset) - 1;
            }

            let start_offset = app.scroll_offset;
            let end_offset = (start_offset + viewport_height).min(logs_len);

            // Calculate max cursor position
            let visible_log_count = end_offset.saturating_sub(start_offset);
            let max_cursor = if end_offset >= logs_len {
                visible_log_count
            } else {
                visible_log_count.saturating_sub(1)
            };

            let cursor = app.viewport_cursor.min(max_cursor);

            // Create list items from visible range
            let mut log_items: Vec<ListItem> = logs[start_offset..end_offset]
                .iter()
                .map(|l| ListItem::new(Self::format_log_line(l, app)))
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
                .highlight_style(Style::default().bg(Color::DarkGray))
                .highlight_symbol(">> ");

            // Set cursor position within the visible list (including EOF)
            let mut state = ListState::default();
            state.select(Some(cursor));

            frame.render_stateful_widget(log_list, chunks[logs_chunk_index], &mut state);
        }
    }
}
