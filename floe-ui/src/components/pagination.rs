//! Pagination component — navigation for paginated data.

use crate::components::button;
use crate::components::icon::{self, IconName};
use crate::theme::tokens::DesignTokens;
use iced::widget::{row, text, Row};
use iced::{Element, Theme};

// ── Builder functions ───────────────────────────────────────────────────

/// Create a pagination container.
pub fn pagination<'a, Message: 'a>(
    items: impl IntoIterator<Item = Element<'a, Message, Theme>>,
) -> Row<'a, Message, Theme> {
    row(items)
        .spacing(4)
        .align_y(iced::Center)
}

/// A standard pagination link/button.
pub fn item<'a, Message: Clone + 'a>(
    label: &str,
    is_active: bool,
    on_press: Message,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    let btn = if is_active {
        button::outline(label, tokens).on_press(on_press)
    } else {
        button::ghost(label, tokens).on_press(on_press)
    };
    btn.into()
}

/// The "Previous" pagination button with chevron icon.
pub fn previous<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    let content = row![
        icon::view(IconName::ChevronLeft).size(16),
        text("Previous").size(14)
    ]
    .spacing(4)
    .align_y(iced::Center);

    let mut btn = iced::widget::button(content)
        .padding([8, 12])
        .style(button::ghost_style(tokens));
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    btn.into()
}

/// The "Next" pagination button with chevron icon.
pub fn next<'a, Message: Clone + 'a>(
    on_press: Option<Message>,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    let content = row![
        text("Next").size(14),
        icon::view(IconName::ChevronRight).size(16)
    ]
    .spacing(4)
    .align_y(iced::Center);

    let mut btn = iced::widget::button(content)
        .padding([8, 12])
        .style(button::ghost_style(tokens));
    
    if let Some(msg) = on_press {
        btn = btn.on_press(msg);
    }
    
    btn.into()
}

/// An ellipsis (...) placeholder for pagination.
pub fn ellipsis<'a, Message: 'a>(
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    iced::widget::container(text("...").size(14).color(tokens.muted_foreground))
        .padding([8, 12])
        .into()
}
