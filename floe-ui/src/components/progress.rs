//! Progress bar component — styled wrapper around Iced's `progress_bar`.

use crate::theme::tokens::DesignTokens;
use iced::widget::progress_bar::Style;
use iced::widget::{progress_bar, ProgressBar};
use iced::{Background, Border, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default progress bar — primary-colored fill on muted track.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Background::Color(tokens.secondary),
        bar: Background::Color(tokens.primary),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
    }
}

/// Destructive progress bar — danger-colored fill.
pub fn destructive_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Background::Color(tokens.secondary),
        bar: Background::Color(tokens.destructive),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
    }
}

/// Success-colored progress bar.
pub fn success_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    let success = iced::Color::from_rgb(0.29, 0.78, 0.47);
    move |_theme| Style {
        background: Background::Color(tokens.secondary),
        bar: Background::Color(success),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a styled progress bar.
pub fn styled<'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    tokens: &'a DesignTokens,
) -> ProgressBar<'a, Theme> {
    progress_bar(range, value)
        .girth(8.0)
        .style(default_style(tokens))
}

/// Create a destructive-styled progress bar.
pub fn destructive<'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    tokens: &'a DesignTokens,
) -> ProgressBar<'a, Theme> {
    progress_bar(range, value)
        .girth(8.0)
        .style(destructive_style(tokens))
}

/// Create a success-styled progress bar.
pub fn success<'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    tokens: &'a DesignTokens,
) -> ProgressBar<'a, Theme> {
    progress_bar(range, value)
        .girth(8.0)
        .style(success_style(tokens))
}
