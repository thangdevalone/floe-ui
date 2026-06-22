//! Table component — structured layout for rows and columns of data.
//!
//! Note: Iced does not have a native table widget, so this is built
//! using structured columns and rows.

use crate::theme::tokens::DesignTokens;
use iced::widget::container::Style;
use iced::widget::{column, container, row, text, Container, Row};
use iced::{Background, Border, Element, Length, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Table container style.
pub fn table_container_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.background)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_md.into(),
        },
        ..Default::default()
    }
}

/// Table header row style.
pub fn header_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        border: Border {
            color: tokens.border,
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

/// Table body row style (with bottom border).
pub fn row_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        border: Border {
            color: tokens.border,
            width: 0.0, // Top/bottom borders are handled by separators in the builder
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a table with headers and rows.
pub fn table<'a, Message: 'a>(
    headers: impl IntoIterator<Item = Element<'a, Message, Theme>>,
    rows: impl IntoIterator<Item = Row<'a, Message, Theme>>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    let header_row = row(headers)
        .spacing(16)
        .padding([12, 16])
        .align_y(iced::Center)
        .width(Length::Fill);

    let mut content = column![
        container(header_row).style(header_style(tokens)),
        crate::components::separator::horizontal(tokens)
    ];

    for r in rows {
        content = content.push(
            container(r.spacing(16).padding([12, 16]).align_y(iced::Center))
                .style(row_style(tokens))
                .width(Length::Fill),
        );
        content = content.push(crate::components::separator::horizontal(tokens));
    }

    container(content).style(table_container_style(tokens))
}

/// Create a standard text cell.
pub fn cell<'a, Message: 'a>(
    content: &str,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    container(
        text(content.to_string())
            .size(14)
            .color(tokens.foreground)
    )
    .width(Length::Fill)
    .into()
}

/// Create a standard header cell.
pub fn header_cell<'a, Message: 'a>(
    content: &str,
    tokens: &'a DesignTokens,
) -> Element<'a, Message, Theme> {
    container(
        text(content.to_string())
            .size(14)
            .color(tokens.muted_foreground)
    )
    .width(Length::Fill)
    .into()
}
