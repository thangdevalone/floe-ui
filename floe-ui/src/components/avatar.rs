//! Avatar component — circular user avatar with initials fallback.
//!
//! Shows a colored circle with initials text when no image is available.

use crate::theme::tokens::DesignTokens;
use iced::widget::container::Style;
use iced::widget::{center, container, text, Container};
use iced::{Background, Border, Color, Shadow, Theme};

/// Avatar size preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvatarSize {
    /// Small: 32×32
    Sm,
    /// Default: 40×40
    Default,
    /// Large: 56×56
    Lg,
}

impl AvatarSize {
    fn dimension(self) -> f32 {
        match self {
            Self::Sm => 32.0,
            Self::Default => 40.0,
            Self::Lg => 56.0,
        }
    }

    fn text_size(self) -> f32 {
        match self {
            Self::Sm => 12.0,
            Self::Default => 16.0,
            Self::Lg => 22.0,
        }
    }
}

// ── Style function ──────────────────────────────────────────────────────

/// Default avatar styling — circular with muted background.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.muted)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_full.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(tokens.muted_foreground),
        snap: false,
    }
}

/// Primary-colored avatar.
pub fn primary_style<'a>(
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

// ── Builder functions ───────────────────────────────────────────────────

/// Create an avatar with initials.
pub fn initials<'a, Message: 'a>(
    name: &str,
    size: AvatarSize,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    let init: String = name
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .collect();

    let dim = size.dimension();

    container(center(text(init).size(size.text_size())))
        .width(dim)
        .height(dim)
        .style(default_style(tokens))
}

/// Create a primary-colored avatar with initials.
pub fn initials_primary<'a, Message: 'a>(
    name: &str,
    size: AvatarSize,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    let init: String = name
        .split_whitespace()
        .take(2)
        .filter_map(|w| w.chars().next())
        .map(|c| c.to_uppercase().to_string())
        .collect();

    let dim = size.dimension();

    container(center(text(init).size(size.text_size())))
        .width(dim)
        .height(dim)
        .style(primary_style(tokens))
}
