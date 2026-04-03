use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::ui::theme;

pub fn render(frame: &mut Frame, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(3), // input area
        Constraint::Min(1),    // results area
    ])
    .split(area);

    // Input bar (placeholder)
    let input_block = Block::default()
        .title(Span::styled(" Domain Lookup ", theme::title_style()))
        .borders(Borders::ALL)
        .border_style(theme::active_border_style());

    let input_hint = Paragraph::new(Line::from(Span::styled(
        "  Domain input coming soon...",
        theme::muted_style(),
    )))
    .block(input_block);

    frame.render_widget(input_hint, chunks[0]);

    // Results area (placeholder)
    let results_block = Block::default()
        .title(Span::styled(" Results ", theme::title_style()))
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .padding(Padding::new(2, 2, 1, 1));

    let placeholder = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Enter a domain name to perform diagnostics",
            theme::muted_style(),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("Available operations: ", theme::muted_style()),
            Span::styled(
                "Lookup",
                ratatui::style::Style::default().fg(theme::SAPPHIRE),
            ),
            Span::styled(" · ", theme::muted_style()),
            Span::styled(
                "WHOIS",
                ratatui::style::Style::default().fg(theme::SAPPHIRE),
            ),
            Span::styled(" · ", theme::muted_style()),
            Span::styled("RDAP", ratatui::style::Style::default().fg(theme::SAPPHIRE)),
            Span::styled(" · ", theme::muted_style()),
            Span::styled("DNS", ratatui::style::Style::default().fg(theme::SAPPHIRE)),
            Span::styled(" · ", theme::muted_style()),
            Span::styled(
                "Status",
                ratatui::style::Style::default().fg(theme::SAPPHIRE),
            ),
            Span::styled(" · ", theme::muted_style()),
            Span::styled(
                "Availability",
                ratatui::style::Style::default().fg(theme::SAPPHIRE),
            ),
        ]),
    ];

    frame.render_widget(
        Paragraph::new(placeholder)
            .block(results_block)
            .alignment(Alignment::Center),
        chunks[1],
    );
}
