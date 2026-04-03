pub mod theme;

mod dashboard;
mod footer;
mod header;
mod seer;
mod tome;

use ratatui::{
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders},
    Frame,
};

use crate::app::{App, Tab};

/// Root render function — lays out header, body, and footer, then dispatches
/// the body to the active tab's renderer.
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Fill background with the base colour.
    let bg = Block::default().style(ratatui::style::Style::default().bg(theme::BASE));
    frame.render_widget(bg, area);

    let chunks = Layout::vertical([
        Constraint::Length(3), // header (title + tabs)
        Constraint::Min(1),    // body
        Constraint::Length(1), // footer
    ])
    .split(area);

    render_header(frame, chunks[0], app);
    render_body(frame, chunks[1], app);
    render_footer(frame, chunks[2]);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    header::render(frame, area, app);
}

fn render_body(frame: &mut Frame, area: Rect, app: &App) {
    // Inset the body by 1 on each side for visual padding.
    let inner = Layout::horizontal([
        Constraint::Length(1),
        Constraint::Min(1),
        Constraint::Length(1),
    ])
    .split(area);

    let body_block = Block::default()
        .borders(Borders::NONE)
        .style(ratatui::style::Style::default().bg(theme::BASE));
    frame.render_widget(body_block, inner[1]);

    match app.active_tab {
        Tab::Dashboard => dashboard::render(frame, inner[1]),
        Tab::Seer => seer::render(frame, inner[1]),
        Tab::Tome => tome::render(frame, inner[1]),
    }
}

fn render_footer(frame: &mut Frame, area: Rect) {
    footer::render(frame, area);
}
