//! Skeleton component — a placeholder for loading state.

use crate::theme::tokens::DesignTokens;
use iced::widget::container::Style;
use iced::widget::{container, Container};
use iced::{Background, Border, Length, Theme};

// ── Style function ──────────────────────────────────────────────────────

/// Default skeleton styling — muted background.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.muted)),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_md.into(),
        },
        ..Default::default()
    }
}

/// Circular skeleton styling (e.g. for avatars).
pub fn circle_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.muted)),
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_full.into(),
        },
        ..Default::default()
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a rectangular skeleton placeholder.
pub fn rect<'a, Message: 'a>(
    width: impl Into<Length>,
    height: impl Into<Length>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(iced::widget::Space::new().width(width).height(height))
        .style(default_style(tokens))
}

/// Create a circular skeleton placeholder.
pub fn circle<'a, Message: 'a>(
    diameter: impl Into<Length>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    let d = diameter.into();
    container(iced::widget::Space::new().width(d).height(d))
        .style(circle_style(tokens))
}
