use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

use crate::ui::theme;

const LOGO: &str = r"
     ╔═══════════════════════════════════════════╗
     ║           A  R  C  A  N  U  M             ║
     ║        Domain Intelligence Suite          ║
     ╚═══════════════════════════════════════════╝
";

pub fn render(frame: &mut Frame, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(7), // logo
        Constraint::Length(8), // suite components
        Constraint::Length(7), // quick start
        Constraint::Min(0),    // spacer
    ])
    .split(area);

    // Logo
    let logo = Paragraph::new(LOGO)
        .style(ratatui::style::Style::default().fg(theme::MAUVE))
        .alignment(Alignment::Center);
    frame.render_widget(logo, chunks[0]);

    // Suite components
    let components = vec![
        Line::from(vec![
            Span::styled("  Seer     ", theme::title_style()),
            Span::styled(
                "v0.16.0  ",
                ratatui::style::Style::default().fg(theme::TEAL),
            ),
            Span::styled(
                "Domain diagnostics (WHOIS / RDAP / DNS)",
                theme::muted_style(),
            ),
        ]),
        Line::from(vec![
            Span::styled("  Tome     ", theme::title_style()),
            Span::styled(
                "v0.1.0   ",
                ratatui::style::Style::default().fg(theme::TEAL),
            ),
            Span::styled(
                "Reference database (TLDs / Records / Glossary)",
                theme::muted_style(),
            ),
        ]),
    ];

    let components_block = Block::default()
        .title(Span::styled(" Suite Components ", theme::title_style()))
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .padding(Padding::new(2, 2, 1, 1));

    frame.render_widget(
        Paragraph::new(components).block(components_block),
        chunks[1],
    );

    // Quick start
    let quick_start = vec![
        Line::from(vec![
            Span::styled("  Tab      ", theme::title_style()),
            Span::styled("Switch between views", theme::muted_style()),
        ]),
        Line::from(vec![
            Span::styled("  1-3      ", theme::title_style()),
            Span::styled("Jump to Dashboard / Seer / Tome", theme::muted_style()),
        ]),
        Line::from(vec![
            Span::styled("  q        ", theme::title_style()),
            Span::styled("Quit", theme::muted_style()),
        ]),
    ];

    let quick_start_block = Block::default()
        .title(Span::styled(" Quick Start ", theme::title_style()))
        .borders(Borders::ALL)
        .border_style(theme::border_style())
        .padding(Padding::new(2, 2, 1, 0));

    frame.render_widget(
        Paragraph::new(quick_start).block(quick_start_block),
        chunks[2],
    );
}
