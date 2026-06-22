//! Checkbox component — styled wrapper around Iced's `checkbox`.

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::checkbox::{Status, Style};
use iced::widget::{checkbox, Checkbox};
use iced::{Background, Border, Color, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default checkbox styling.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let (bg, border_color, icon_color) = match status {
            Status::Active { is_checked } => {
                if is_checked {
                    (tokens.primary, tokens.primary, tokens.primary_foreground)
                } else {
                    (Color::TRANSPARENT, tokens.input, tokens.primary_foreground)
                }
            }
            Status::Hovered { is_checked } => {
                if is_checked {
                    (
                        with_alpha(tokens.primary, 0.9),
                        with_alpha(tokens.primary, 0.9),
                        tokens.primary_foreground,
                    )
                } else {
                    (Color::TRANSPARENT, tokens.ring, tokens.primary_foreground)
                }
            }
            Status::Disabled { is_checked } => {
                if is_checked {
                    (
                        with_alpha(tokens.primary, 0.5),
                        with_alpha(tokens.primary, 0.5),
                        with_alpha(tokens.primary_foreground, 0.5),
                    )
                } else {
                    (
                        Color::TRANSPARENT,
                        with_alpha(tokens.input, 0.5),
                        with_alpha(tokens.primary_foreground, 0.5),
                    )
                }
            }
        };

        Style {
            background: Background::Color(bg),
            icon_color,
            border: Border {
                color: border_color,
                width: 1.5,
                radius: tokens.radius_sm.into(),
            },
            text_color: Some(tokens.foreground),
        }
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Create a styled checkbox.
///
/// In Iced 0.14, `checkbox(is_checked)` takes only the checked state.
/// The label is set separately via `.label()`.
pub fn styled<'a, Message: Clone + 'a>(
    label: &'a str,
    is_checked: bool,
    tokens: &'a DesignTokens,
) -> Checkbox<'a, Message, Theme> {
    checkbox(is_checked)
        .label(label)
        .size(18.0)
        .spacing(8.0)
        .style(default_style(tokens))
}
