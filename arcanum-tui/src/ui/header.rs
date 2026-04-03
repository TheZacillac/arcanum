use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Tabs},
    Frame,
};

use crate::app::{App, Tab};
use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::vertical([
        Constraint::Length(1), // title
        Constraint::Length(2), // tabs (including top border)
    ])
    .split(area);

    // Title line
    let title = Line::from(vec![
        Span::styled(" Arcanum", theme::title_style()),
        Span::styled(
            format!("  v{}", env!("CARGO_PKG_VERSION")),
            theme::muted_style(),
        ),
    ]);
    frame.render_widget(Paragraph::new(title), chunks[0]);

    // Tab bar
    let tab_titles: Vec<Line> = Tab::ALL
        .iter()
        .map(|t| {
            let style = if *t == app.active_tab {
                theme::tab_active_style()
            } else {
                theme::tab_inactive_style()
            };
            Line::from(Span::styled(format!(" {} ", t.title()), style))
        })
        .collect();

    let tabs = Tabs::new(tab_titles)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(theme::border_style())
                .padding(Padding::horizontal(1)),
        )
        .select(app.active_tab.index())
        .highlight_style(theme::tab_active_style().add_modifier(Modifier::BOLD))
        .divider(Span::styled(" │ ", theme::border_style()));

    frame.render_widget(tabs, chunks[1]);
}
