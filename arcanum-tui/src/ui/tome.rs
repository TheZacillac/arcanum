use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let chunks = Layout::horizontal([
        Constraint::Percentage(30), // navigation sidebar
        Constraint::Percentage(70), // detail panel
    ])
    .split(area);

    // Sidebar — section navigation
    let sections = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  TLDs",
            ratatui::style::Style::default().fg(theme::SAPPHIRE),
        )),
        Line::from(Span::styled(
            "    Top-level domain reference",
            theme::muted_style(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Record Types",
            ratatui::style::Style::default().fg(theme::SAPPHIRE),
        )),
        Line::from(Span::styled(
            "    DNS record type definitions",
            theme::muted_style(),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Glossary",
            ratatui::style::Style::default().fg(theme::SAPPHIRE),
        )),
        Line::from(Span::styled(
            "    Domain industry terminology",
            theme::muted_style(),
        )),
    ];

    let sidebar_block = Block::default()
        .title(Span::styled(" Sections ", theme::title_style()))
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .padding(Padding::new(1, 1, 0, 0));

    frame.render_widget(Paragraph::new(sections).block(sidebar_block), chunks[0]);

    // Detail panel (placeholder)
    let detail_block = Block::default()
        .title(Span::styled(" Detail ", theme::title_style()))
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .padding(Padding::new(2, 2, 1, 1));

    let placeholder = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Select a section to browse the reference database",
            theme::muted_style(),
        )),
    ];

    frame.render_widget(
        Paragraph::new(placeholder)
            .block(detail_block)
            .alignment(Alignment::Center),
        chunks[1],
    );
}
