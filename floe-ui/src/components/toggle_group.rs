//! Toggle Group component — a set of interconnected toggle buttons.

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::button::{Status, Style};
use iced::widget::{button as button_widget, row, Row};
use iced::border::Radius;
use iced::{Background, Border, Element, Theme};

/// Defines the position of an item in a ToggleGroup to apply correct border radii.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemPosition {
    First,
    Middle,
    Last,
    Single,
}

// ── Style functions ─────────────────────────────────────────────────────

/// Style for an active (pressed/selected) toggle group item.
pub fn item_active_style<'a>(
    position: ItemPosition,
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, _status| {
        let radius = match position {
            ItemPosition::First => Radius { top_left: tokens.radius_md, top_right: 0.0, bottom_right: 0.0, bottom_left: tokens.radius_md },
            ItemPosition::Middle => Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 },
            ItemPosition::Last => Radius { top_left: 0.0, top_right: tokens.radius_md, bottom_right: tokens.radius_md, bottom_left: 0.0 },
            ItemPosition::Single => Radius { top_left: tokens.radius_md, top_right: tokens.radius_md, bottom_right: tokens.radius_md, bottom_left: tokens.radius_md },
        };

        Style {
            background: Some(Background::Color(tokens.accent)),
            text_color: tokens.accent_foreground,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius,
            },
            ..Default::default()
        }
    }
}

/// Style for an inactive (unselected) toggle group item.
pub fn item_inactive_style<'a>(
    position: ItemPosition,
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let radius = match position {
            ItemPosition::First => Radius { top_left: tokens.radius_md, top_right: 0.0, bottom_right: 0.0, bottom_left: tokens.radius_md },
            ItemPosition::Middle => Radius { top_left: 0.0, top_right: 0.0, bottom_right: 0.0, bottom_left: 0.0 },
            ItemPosition::Last => Radius { top_left: 0.0, top_right: tokens.radius_md, bottom_right: tokens.radius_md, bottom_left: 0.0 },
            ItemPosition::Single => Radius { top_left: tokens.radius_md, top_right: tokens.radius_md, bottom_right: tokens.radius_md, bottom_left: tokens.radius_md },
        };

        let (bg, text_color) = match status {
            Status::Hovered => (
                Some(Background::Color(with_alpha(tokens.accent, 0.5))),
                tokens.accent_foreground,
            ),
            _ => (Some(Background::Color(iced::Color::TRANSPARENT)), tokens.foreground),
        };

        Style {
            background: bg,
            text_color,
            border: Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius,
            },
            ..Default::default()
        }
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a toggle group container.
pub fn group<'a, Message: 'a>(
    items: impl IntoIterator<Item = Element<'a, Message, Theme>>,
) -> Row<'a, Message, Theme> {
    row(items).spacing(0) // 0 spacing so items connect
}

/// Create an item for a toggle group.
pub fn item<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    is_active: bool,
    position: ItemPosition,
    on_press: Message,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    let btn = button_widget(content)
        .padding([8, 12])
        .on_press(on_press);

    if is_active {
        btn.style(item_active_style(position, tokens)).into()
    } else {
        btn.style(item_inactive_style(position, tokens)).into()
    }
}
