use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};

mod app;
mod log;
mod ui;

use crate::{
    app::{App, InputMode},
    log::LogReader,
};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Initialize App State
    let mut app = App::new();
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    // 3. Main Event Loop
    while app.running {
        terminal.draw(|f| ui::render(&mut app, f))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());

        let mut current_log_task: Option<tokio::task::JoinHandle<()>> = None;

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                handle_key_event(&mut app, key, &mut current_log_task);
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // 4. Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_key_event(
    app: &mut App,
    key: KeyEvent,
    current_log_task: &mut Option<tokio::task::JoinHandle<()>>,
) {
    // Only process Press events, ignore Release and Repeat
    if key.kind != KeyEventKind::Press {
        return;
    }

    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('e') => {
                // Press 'e' to edit path
                app.input_mode = InputMode::Edit;
            }
            KeyCode::Char('q') => {
                app.quit();
            }
            KeyCode::Char('k') => app.scroll_up(),
            KeyCode::Char('j') => app.scroll_down(),
            _ => {}
        },
        InputMode::Edit => match key.code {
            KeyCode::Enter => {
                let path = app.log_file_input.drain(..).collect::<String>();

                // Clear previous error
                app.clear_error();

                // Validate file path is not empty
                if path.trim().is_empty() {
                    app.set_error("File path cannot be empty".to_string());
                    app.input_mode = InputMode::Normal;
                    return;
                }

                // Check if file exists
                if !std::path::Path::new(&path).exists() {
                    app.set_error(format!("File not found: '{}'", path));
                    app.input_mode = InputMode::Normal;
                    return;
                }

                // Check if it's actually a file (not a directory)
                match std::path::Path::new(&path).metadata() {
                    Ok(metadata) if !metadata.is_file() => {
                        app.set_error(format!("'{}' is not a file", path));
                        app.input_mode = InputMode::Normal;
                        return;
                    }
                    Err(e) => {
                        app.set_error(format!("Cannot access file '{}': {}", path, e));
                        app.input_mode = InputMode::Normal;
                        return;
                    }
                    _ => {}
                }

                // All validation passed, proceed with loading the file
                if let Some(handle) = current_log_task.take() {
                    handle.abort(); // Kill old log reader task.
                }

                app.current_path = path.clone();
                app.input_mode = InputMode::Normal;

                let logs_ptr = app.logs.clone();
                *current_log_task = Some(tokio::spawn(async move {
                    // Clear old logs when switching files
                    logs_ptr.lock().await.clear();
                    let _ = LogReader::tail_file(&path, logs_ptr).await;
                }));
            }
            KeyCode::Char(c) => {
                app.log_file_input.push(c);
            }
            KeyCode::Backspace => {
                app.log_file_input.pop();
            }
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            _ => {}
        },
    }
}
