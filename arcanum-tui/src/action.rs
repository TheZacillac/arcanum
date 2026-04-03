use crate::app::Tab;

/// All possible user-initiated actions, decoupled from key bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Quit the application.
    Quit,

    /// Cycle to the next tab.
    NextTab,

    /// Cycle to the previous tab.
    PrevTab,

    /// Jump directly to a tab.
    GoToTab(Tab),

    /// No-op (unhandled key).
    None,
}
