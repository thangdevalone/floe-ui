//! Select component — styled wrapper around Iced's `pick_list`.

use crate::theme::tokens::DesignTokens;
use iced::widget::pick_list::{Status, Style};
use iced::widget::{pick_list, PickList};
use iced::{Background, Border, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default select/dropdown styling.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let base = Style {
            background: Background::Color(tokens.background),
            text_color: tokens.foreground,
            placeholder_color: tokens.muted_foreground,
            handle_color: tokens.muted_foreground,
            border: Border {
                color: tokens.input,
                width: 1.0,
                radius: tokens.radius_md.into(),
            },
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
            Status::Opened { is_hovered: _ } => Style {
                border: Border {
                    color: tokens.ring,
                    width: 2.0,
                    ..base.border
                },
                ..base
            },
        }
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Create a styled select / dropdown.
pub fn styled<'a, T, L, V, Message>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
    tokens: &'a DesignTokens,
) -> PickList<'a, T, L, V, Message, Theme>
where
    T: ToString + PartialEq + Clone + 'a,
    L: std::borrow::Borrow<[T]> + 'a,
    V: std::borrow::Borrow<T> + 'a,
    Message: Clone + 'a,
{
    pick_list(options, selected, on_selected)
        .padding([8, 12])
        .text_size(14.0)
        .style(default_style(tokens))
}
