use super::TabState;
use crate::app::App;
use crate::port::{PortInfo, PortScanner};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

/// PortScannerTab - The state for the port scanner tab
pub struct PortScannerTab {
    pub scan_results: Vec<PortInfo>,
    pub list_state: ListState,
    pub last_scan_count: usize,
}

impl PortScannerTab {
    pub fn new() -> Self {
        Self {
            scan_results: Vec::new(),
            list_state: ListState::default(),
            last_scan_count: 0,
        }
    }

    pub fn perform_scan(&mut self) {
        self.scan_results = PortScanner::scan_local_ports();
        self.last_scan_count = self.scan_results.len();
        // Sort by port number for better display
        self.scan_results.sort_by_key(|r| r.port);
    }

    pub fn move_selection_up(&mut self) {
        if !self.scan_results.is_empty() {
            match self.list_state.selected() {
                Some(0) => self.list_state.select(Some(self.scan_results.len() - 1)),
                Some(idx) => self.list_state.select(Some(idx - 1)),
                None => self.list_state.select(Some(0)),
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        if !self.scan_results.is_empty() {
            match self.list_state.selected() {
                Some(idx) if idx >= self.scan_results.len() - 1 => self.list_state.select(Some(0)),
                Some(idx) => self.list_state.select(Some(idx + 1)),
                None => self.list_state.select(Some(0)),
            }
        }
    }
}

impl Default for PortScannerTab {
    fn default() -> Self {
        Self::new()
    }
}

impl TabState for PortScannerTab {
    fn name(&self) -> &'static str {
        "Port Scanner"
    }

    fn on_enter(&mut self, _app: &mut App) {
        // Perform scan when entering the tab
        self.perform_scan();
    }

    fn render(&mut self, _app: &mut App, frame: &mut ratatui::Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Min(5), // Results list
            ])
            .split(area);

        self.render_results(frame, chunks[0]);
    }

    fn help_text(&self) -> Option<&str> {
        Some("Port Scanner - Shows open ports on localhost. Press Up/Down to navigate results.")
    }
}

impl PortScannerTab {
    fn render_results(&mut self, frame: &mut ratatui::Frame, area: Rect) {
        if self.scan_results.is_empty() {
            let placeholder = Paragraph::new("No open ports found on localhost")
                .style(Style::default().fg(Color::DarkGray))
                .block(Block::default().borders(Borders::ALL).title(" Results "));
            frame.render_widget(placeholder, area);
            return;
        }

        let items: Vec<ListItem> = self
            .scan_results
            .iter()
            .map(|port_info| {
                let port_span = Span::styled(
                    format!("{:>5}", port_info.port),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );

                let protocol_span = Span::styled(
                    format!("{:<6}", port_info.protocol),
                    Style::default().fg(Color::Cyan),
                );

                let process_span = Span::raw(format!(
                    "  {}",
                    if port_info.process_name == "Unknown" {
                        port_info.process_name.clone()
                    } else {
                        format!("PID: {}", port_info.pid.unwrap_or(0))
                    }
                ));

                ListItem::new(Line::from(vec![
                    port_span,
                    Span::raw("  "),
                    protocol_span,
                    process_span,
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" Results ({} ports) ", self.scan_results.len())),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }
}
