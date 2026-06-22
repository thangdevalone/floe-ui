//! Badge component — small status labels / tags.
//!
//! Mirrors shadcn/ui's Badge component with variants:
//! `Default`, `Secondary`, `Outline`, `Destructive`.

use crate::theme::tokens::DesignTokens;
use iced::widget::container::Style;
use iced::widget::{container, text, Container};
use iced::{Background, Border, Color, Shadow, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default badge — filled with primary color.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.primary)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(tokens.primary_foreground),
        snap: false,
    }
}

/// Secondary badge — muted background.
pub fn secondary_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.secondary)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(tokens.secondary_foreground),
        snap: false,
    }
}

/// Outline badge — border only, transparent background.
pub fn outline_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_full.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(tokens.foreground),
        snap: false,
    }
}

/// Destructive badge — danger color.
pub fn destructive_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.destructive)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(tokens.destructive_foreground),
        snap: false,
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a **default** badge.
pub fn primary<'a, Message: 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(text(label.to_string()).size(12.0))
        .padding([2, 10])
        .style(default_style(tokens))
}

/// Create a **secondary** badge.
pub fn secondary<'a, Message: 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(text(label.to_string()).size(12.0))
        .padding([2, 10])
        .style(secondary_style(tokens))
}

/// Create an **outline** badge.
pub fn outline<'a, Message: 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(text(label.to_string()).size(12.0))
        .padding([2, 10])
        .style(outline_style(tokens))
}

/// Create a **destructive** badge.
pub fn destructive<'a, Message: 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(text(label.to_string()).size(12.0))
        .padding([2, 10])
        .style(destructive_style(tokens))
}
