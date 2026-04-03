use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let hints = Line::from(vec![
        Span::styled(" Tab", theme::title_style()),
        Span::styled(" Switch   ", theme::muted_style()),
        Span::styled("1-3", theme::title_style()),
        Span::styled(" Jump   ", theme::muted_style()),
        Span::styled("q", theme::title_style()),
        Span::styled(" Quit", theme::muted_style()),
    ]);

    frame.render_widget(
        Paragraph::new(hints).style(ratatui::style::Style::default().bg(theme::MANTLE)),
        area,
    );
}
