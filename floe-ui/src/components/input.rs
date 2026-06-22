//! Styled text input with focus ring, border colors, and placeholder
//! styling driven by [`DesignTokens`].

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::text_input::{Status, Style};
use iced::widget::{text_input, TextInput};
use iced::{Background, Border, Color, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default input styling.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let base = Style {
            background: Background::Color(tokens.background),
            border: Border {
                color: tokens.input,
                width: 1.0,
                radius: tokens.radius_md.into(),
            },
            icon: tokens.muted_foreground,
            placeholder: tokens.muted_foreground,
            value: tokens.foreground,
            selection: with_alpha(tokens.primary, 0.3),
        };

        match status {
            Status::Active => base,
            Status::Hovered => Style {
                border: Border {
                    color: tokens.ring,
                    ..base.border
                },
                ..base
            },
            Status::Focused { is_hovered: _ } => Style {
                border: Border {
                    color: tokens.ring,
                    width: 2.0,
                    ..base.border
                },
                ..base
            },
            Status::Disabled => Style {
                background: Background::Color(tokens.muted),
                border: Border {
                    color: with_alpha(tokens.input, 0.5),
                    ..base.border
                },
                placeholder: with_alpha(tokens.muted_foreground, 0.5),
                value: with_alpha(tokens.foreground, 0.5),
                ..base
            },
        }
    }
}

/// Ghost input — minimal border, blends into the background.
pub fn ghost_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let base = Style {
            background: Background::Color(Color::TRANSPARENT),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: tokens.radius_md.into(),
            },
            icon: tokens.muted_foreground,
            placeholder: tokens.muted_foreground,
            value: tokens.foreground,
            selection: with_alpha(tokens.primary, 0.3),
        };

        match status {
            Status::Active => base,
            Status::Hovered => Style {
                background: Background::Color(with_alpha(tokens.accent, 0.5)),
                ..base
            },
            Status::Focused { is_hovered: _ } => Style {
                background: Background::Color(with_alpha(tokens.accent, 0.5)),
                border: Border {
                    color: tokens.ring,
                    width: 1.0,
                    ..base.border
                },
                ..base
            },
            Status::Disabled => Style {
                placeholder: with_alpha(tokens.muted_foreground, 0.5),
                value: with_alpha(tokens.foreground, 0.5),
                ..base
            },
        }
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a styled text input with the **default** variant.
pub fn styled<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    tokens: &'a DesignTokens,
) -> TextInput<'a, Message, Theme> {
    text_input(placeholder, value)
        .padding([8, 12])
        .size(14.0)
        .style(default_style(tokens))
}

/// Create a **ghost** text input (minimal styling).
pub fn ghost<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
    tokens: &'a DesignTokens,
) -> TextInput<'a, Message, Theme> {
    text_input(placeholder, value)
        .padding([8, 12])
        .size(14.0)
        .style(ghost_style(tokens))
}
