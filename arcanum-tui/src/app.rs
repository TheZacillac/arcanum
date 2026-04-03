use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::action::Action;

/// The available top-level tabs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Dashboard,
    Seer,
    Tome,
}

impl Tab {
    pub const ALL: [Tab; 3] = [Tab::Dashboard, Tab::Seer, Tab::Tome];

    pub fn title(self) -> &'static str {
        match self {
            Tab::Dashboard => "Dashboard",
            Tab::Seer => "Seer",
            Tab::Tome => "Tome",
        }
    }

    pub fn index(self) -> usize {
        match self {
            Tab::Dashboard => 0,
            Tab::Seer => 1,
            Tab::Tome => 2,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i % Self::ALL.len() {
            0 => Tab::Dashboard,
            1 => Tab::Seer,
            _ => Tab::Tome,
        }
    }
}

/// Central application state.
pub struct App {
    /// Whether the app should exit on the next loop iteration.
    pub should_quit: bool,

    /// Currently active tab.
    pub active_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            active_tab: Tab::Dashboard,
        }
    }

    /// Map a key event to an action.
    pub fn handle_key(&self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Char('q') => Action::Quit,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => Action::Quit,

            // Tab navigation
            KeyCode::Tab => Action::NextTab,
            KeyCode::BackTab => Action::PrevTab,
            KeyCode::Char('1') => Action::GoToTab(Tab::Dashboard),
            KeyCode::Char('2') => Action::GoToTab(Tab::Seer),
            KeyCode::Char('3') => Action::GoToTab(Tab::Tome),

            _ => Action::None,
        }
    }

    /// Apply an action to the app state.
    pub fn dispatch(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,

            Action::NextTab => {
                let next = (self.active_tab.index() + 1) % Tab::ALL.len();
                self.active_tab = Tab::from_index(next);
            }
            Action::PrevTab => {
                let prev = (self.active_tab.index() + Tab::ALL.len() - 1) % Tab::ALL.len();
                self.active_tab = Tab::from_index(prev);
            }
            Action::GoToTab(tab) => self.active_tab = tab,

            Action::None => {}
        }
    }

    /// Called on every tick (for future animations / status polling).
    pub fn on_tick(&mut self) {
        // No-op for now.
    }
}
