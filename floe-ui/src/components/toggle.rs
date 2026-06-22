//! Toggle / Switch component — styled wrapper around Iced's `toggler`.

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::toggler::{Status, Style};
use iced::widget::{toggler, Toggler};
use iced::{Background, Color, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default toggle styling.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let (track_bg, knob_bg, opacity) = match status {
            Status::Active { is_toggled } => {
                if is_toggled {
                    (tokens.primary, tokens.primary_foreground, 1.0)
                } else {
                    (tokens.input, tokens.foreground, 1.0)
                }
            }
            Status::Hovered { is_toggled } => {
                if is_toggled {
                    (tokens.primary, tokens.primary_foreground, 0.85)
                } else {
                    (tokens.input, tokens.foreground, 0.85)
                }
            }
            Status::Disabled { is_toggled } => {
                if is_toggled {
                    (tokens.primary, tokens.primary_foreground, 0.5)
                } else {
                    (tokens.input, tokens.foreground, 0.5)
                }
            }
        };

        Style {
            background: Background::Color(with_alpha(track_bg, opacity)),
            background_border_width: 0.0,
            background_border_color: Color::TRANSPARENT,
            foreground: Background::Color(with_alpha(knob_bg, opacity)),
            foreground_border_width: 0.0,
            foreground_border_color: Color::TRANSPARENT,
            border_radius: Some(10.0.into()),
            padding_ratio: 0.1,
            text_color: Some(tokens.foreground),
        }
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Create a styled toggle (switch).
pub fn styled<'a, Message: 'a>(
    label: impl Into<Option<String>>,
    is_toggled: bool,
    tokens: &'a DesignTokens,
) -> Toggler<'a, Message, Theme> {
    let label_text: Option<String> = label.into();
    let mut t = toggler(is_toggled)
        .size(20.0)
        .style(default_style(tokens));
    if let Some(l) = label_text {
        t = t.label(l);
    }
    t
}
