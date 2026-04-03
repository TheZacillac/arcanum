use std::time::Duration;

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use tokio::sync::mpsc;

/// Events produced by the terminal and internal timers.
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppEvent {
    /// A key was pressed.
    Key(KeyEvent),
    /// A mouse event occurred.
    Mouse(MouseEvent),
    /// The terminal was resized.
    Resize(u16, u16),
    /// Periodic tick for UI updates.
    Tick,
}

/// Polls crossterm events and emits them on a channel.
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<AppEvent>,
    _tx: mpsc::UnboundedSender<AppEvent>,
}

impl EventHandler {
    /// Spawn the event loop with the given tick rate.
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let event_tx = tx.clone();

        // Use spawn_blocking to avoid blocking the async executor with
        // crossterm's synchronous event::poll / event::read calls.
        std::thread::spawn(move || {
            loop {
                let has_event = event::poll(tick_rate).unwrap_or(false);

                if has_event {
                    match event::read() {
                        Ok(CrosstermEvent::Key(key)) => {
                            if event_tx.send(AppEvent::Key(key)).is_err() {
                                return;
                            }
                        }
                        Ok(CrosstermEvent::Mouse(mouse)) => {
                            if event_tx.send(AppEvent::Mouse(mouse)).is_err() {
                                return;
                            }
                        }
                        Ok(CrosstermEvent::Resize(w, h)) => {
                            if event_tx.send(AppEvent::Resize(w, h)).is_err() {
                                return;
                            }
                        }
                        _ => {}
                    }
                } else {
                    // No event within tick_rate — emit a tick.
                    if event_tx.send(AppEvent::Tick).is_err() {
                        return;
                    }
                }
            }
        });

        Self { rx, _tx: tx }
    }

    /// Wait for the next event.
    pub async fn next(&mut self) -> crate::error::Result<AppEvent> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| crate::error::ArcanumError::Terminal("Event channel closed".into()))
    }
}
