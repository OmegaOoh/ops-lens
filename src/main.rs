use crossterm::{
    event::{self, Event},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    time::{Duration, Instant},
};

mod app;
mod keybinds;
mod log;
mod ui;

use crate::{app::App, keybinds::handle_key_event};

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
                handle_key_event(&mut app, key, &mut current_log_task).await;
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
