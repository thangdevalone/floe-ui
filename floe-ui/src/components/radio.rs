//! Radio component — styled wrapper around Iced's `radio`.

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::radio::{Status, Style};
use iced::widget::{radio, Radio};
use iced::{Background, Color, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default radio styling — circular with primary dot when selected.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let (bg, border_color, dot_color) = match status {
            Status::Active { is_selected } => {
                if is_selected {
                    (tokens.primary, tokens.primary, tokens.primary_foreground)
                } else {
                    (Color::TRANSPARENT, tokens.input, tokens.primary)
                }
            }
            Status::Hovered { is_selected } => {
                if is_selected {
                    (
                        with_alpha(tokens.primary, 0.9),
                        with_alpha(tokens.primary, 0.9),
                        tokens.primary_foreground,
                    )
                } else {
                    (Color::TRANSPARENT, tokens.ring, tokens.primary)
                }
            }
        };

        Style {
            background: Background::Color(bg),
            dot_color,
            border_width: 1.5,
            border_color,
            text_color: Some(tokens.foreground),
        }
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Create a styled radio button.
pub fn styled<'a, Message: Clone + PartialEq + 'a, V: Copy + Eq + 'a>(
    label: &str,
    value: V,
    selected: Option<V>,
    tokens: &'a DesignTokens,
) -> Radio<'a, Message, Theme> {
    radio(label, value, selected, |_| unreachable!())
        .size(18.0)
        .spacing(8.0)
        .style(default_style(tokens))
}

/// Create a styled radio button with an `on_click` handler.
pub fn styled_with<'a, Message: Clone + 'a, V: Copy + Eq + 'a>(
    label: &str,
    value: V,
    selected: Option<V>,
    on_click: Message,
    tokens: &'a DesignTokens,
) -> Radio<'a, Message, Theme> {
    radio(label, value, selected, move |_| on_click.clone())
        .size(18.0)
        .spacing(8.0)
        .style(default_style(tokens))
}
