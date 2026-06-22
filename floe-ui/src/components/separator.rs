//! Separator / Divider component — styled wrapper around Iced's `rule`.

use crate::theme::tokens::DesignTokens;
use iced::widget::rule::{self, Style};
use iced::widget::Rule;
use iced::Theme;

// ── Style function ──────────────────────────────────────────────────────

/// Default separator styling — thin line using `tokens.border` color.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        color: tokens.border,
        radius: 0.0.into(),
        fill_mode: rule::FillMode::Full,
        snap: false,
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a horizontal separator.
pub fn horizontal<'a>(tokens: &'a DesignTokens) -> Rule<'a, Theme> {
    rule::horizontal(1).style(default_style(tokens))
}

/// Create a vertical separator.
pub fn vertical<'a>(tokens: &'a DesignTokens) -> Rule<'a, Theme> {
    rule::vertical(1).style(default_style(tokens))
}
