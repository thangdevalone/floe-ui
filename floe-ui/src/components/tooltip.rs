//! Tooltip component — styled wrapper around Iced's `tooltip`.

use crate::theme::tokens::DesignTokens;
use iced::widget::{container, tooltip as iced_tooltip};
use iced::widget::tooltip::{Position, Tooltip};
use iced::{Background, Border, Color, Element, Shadow, Theme, Vector};

// ── Style function ──────────────────────────────────────────────────────

/// Default tooltip styling — dark popover with text.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> container::Style + 'a {
    move |_theme| container::Style {
        background: Some(Background::Color(tokens.popover)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_md.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.12),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 6.0,
        },
        text_color: Some(tokens.popover_foreground),
        snap: false,
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Wrap a widget with a styled tooltip.
pub fn styled<'a, Message: 'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    tip: impl ToString,
    position: Position,
    tokens: &'a DesignTokens,
) -> Tooltip<'a, Message, Theme> {
    iced_tooltip(content, iced::widget::text(tip.to_string()).size(12), position)
        .padding(6)
        .gap(4.0)
        .style(default_style(tokens))
}
