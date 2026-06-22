//! Tabs component — navigation tabs for switching views.

use crate::theme::tokens::DesignTokens;
use iced::widget::container::Style;
use iced::widget::{button, container, text, Row};
use iced::{Background, Border, Element, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// The container holding all tabs.
pub fn tab_list_style<'a>(
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

/// An active tab button.
pub fn tab_active_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, button::Status) -> button::Style + 'a {
    move |_theme, _status| button::Style {
        background: Some(Background::Color(tokens.background)),
        text_color: tokens.foreground,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_sm.into(),
        },
        shadow: iced::Shadow {
            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            offset: iced::Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
        ..Default::default()
    }
}

/// An inactive tab button.
pub fn tab_inactive_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, button::Status) -> button::Style + 'a {
    move |_theme, status| {
        let text_color = match status {
            button::Status::Hovered => tokens.foreground,
            _ => tokens.muted_foreground,
        };
        button::Style {
            background: Some(Background::Color(iced::Color::TRANSPARENT)),
            text_color,
            border: Border::default(),
            shadow: iced::Shadow::default(),
            ..Default::default()
        }
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a tab list container.
pub fn list<'a, Message: 'a>(
    tabs: impl IntoIterator<Item = Element<'a, Message, Theme>>,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    container(
        Row::with_children(tabs)
            .spacing(4)
            .padding(4),
    )
    .style(tab_list_style(tokens))
    .into()
}

/// Create an individual tab.
pub fn tab<'a, Message: Clone + 'a>(
    label: &str,
    is_active: bool,
    on_press: Message,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    let btn = iced::widget::button(
        container(text(label.to_string()).size(14))
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Shrink),
    )
    .padding([6, 16])
    .on_press(on_press);

    if is_active {
        btn.style(tab_active_style(tokens)).into()
    } else {
        btn.style(tab_inactive_style(tokens)).into()
    }
}
