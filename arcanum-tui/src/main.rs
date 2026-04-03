mod action;
mod app;
mod error;
mod event;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use event::{AppEvent, EventHandler};

/// Tick rate for the event loop (4 Hz).
const TICK_RATE: Duration = Duration::from_millis(250);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialise tracing — respects ARCANUM_LOG_LEVEL, ARCANUM_LOG_FORMAT,
    // ARCANUM_LOG_FILE. Falls back to RUST_LOG for backward compatibility.
    let _log_guard = seer_core::logging::init_logging("arcanum-tui");

    // Install a panic hook that restores the terminal before printing.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let _ = restore_terminal();
        original_hook(panic_info);
    }));

    // Set up terminal.
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Run the app.
    let result = run(&mut terminal).await;

    // Restore terminal no matter what.
    restore_terminal()?;

    result
}

async fn run(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> anyhow::Result<()> {
    let mut app = App::new();
    let mut events = EventHandler::new(TICK_RATE);

    loop {
        // Draw.
        terminal.draw(|frame| ui::render(frame, &app))?;

        // Handle next event.
        match events.next().await? {
            AppEvent::Key(key) => {
                // Ignore key release / repeat events on platforms that emit them.
                if key.kind == crossterm::event::KeyEventKind::Press {
                    let action = app.handle_key(key);
                    app.dispatch(action);
                }
            }
            AppEvent::Tick => {
                app.on_tick();
            }
            AppEvent::Resize(_, _) | AppEvent::Mouse(_) => {
                // Ratatui redraws on next loop iteration automatically.
            }
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn restore_terminal() -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}
