#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};

// ---------------------------------------------------------------------------
// Catppuccin Frappe palette
// https://github.com/catppuccin/catppuccin
// ---------------------------------------------------------------------------

// Accent colours
pub const ROSEWATER: Color = Color::Rgb(242, 213, 207);
pub const FLAMINGO: Color = Color::Rgb(238, 190, 190);
pub const PINK: Color = Color::Rgb(244, 184, 228);
pub const MAUVE: Color = Color::Rgb(202, 158, 230);
pub const RED: Color = Color::Rgb(231, 130, 132);
pub const MAROON: Color = Color::Rgb(234, 153, 156);
pub const PEACH: Color = Color::Rgb(239, 159, 118);
pub const YELLOW: Color = Color::Rgb(229, 200, 144);
pub const GREEN: Color = Color::Rgb(166, 209, 137);
pub const TEAL: Color = Color::Rgb(129, 200, 190);
pub const SKY: Color = Color::Rgb(153, 209, 219);
pub const SAPPHIRE: Color = Color::Rgb(133, 193, 220);
pub const BLUE: Color = Color::Rgb(140, 170, 238);
pub const LAVENDER: Color = Color::Rgb(186, 187, 241);

// Text
pub const TEXT: Color = Color::Rgb(198, 208, 245);
pub const SUBTEXT1: Color = Color::Rgb(181, 191, 226);
pub const SUBTEXT0: Color = Color::Rgb(165, 173, 206);

// Overlays
pub const OVERLAY2: Color = Color::Rgb(148, 156, 187);
pub const OVERLAY1: Color = Color::Rgb(131, 139, 167);
pub const OVERLAY0: Color = Color::Rgb(115, 121, 148);

// Surfaces
pub const SURFACE2: Color = Color::Rgb(98, 104, 128);
pub const SURFACE1: Color = Color::Rgb(81, 87, 109);
pub const SURFACE0: Color = Color::Rgb(65, 69, 89);

// Backgrounds
pub const BASE: Color = Color::Rgb(48, 52, 70);
pub const MANTLE: Color = Color::Rgb(41, 44, 60);
pub const CRUST: Color = Color::Rgb(35, 38, 52);

// ---------------------------------------------------------------------------
// Convenience styles
// ---------------------------------------------------------------------------

pub fn title_style() -> Style {
    Style::default().fg(LAVENDER).add_modifier(Modifier::BOLD)
}

pub fn selected_style() -> Style {
    Style::default().fg(BASE).bg(MAUVE)
}

pub fn border_style() -> Style {
    Style::default().fg(SURFACE2)
}

pub fn active_border_style() -> Style {
    Style::default().fg(BLUE)
}

pub fn error_style() -> Style {
    Style::default().fg(RED)
}

pub fn success_style() -> Style {
    Style::default().fg(GREEN)
}

pub fn warning_style() -> Style {
    Style::default().fg(YELLOW)
}

pub fn muted_style() -> Style {
    Style::default().fg(OVERLAY0)
}

pub fn tab_active_style() -> Style {
    Style::default()
        .fg(BASE)
        .bg(MAUVE)
        .add_modifier(Modifier::BOLD)
}

pub fn tab_inactive_style() -> Style {
    Style::default().fg(SUBTEXT0)
}
