//! Card component — a styled container with optional sections.

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::container::Style;
use iced::widget::{container, Column, Container};
use iced::{Background, Border, Color, Element, Padding, Shadow, Theme, Vector};

// ── Style functions ─────────────────────────────────────────────────────

/// Default card styling: raised surface with subtle border and shadow.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.card)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_lg.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 8.0,
        },
        text_color: Some(tokens.card_foreground),
        snap: false,
    }
}

/// Elevated card — stronger shadow for floating surfaces.
pub fn elevated_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(tokens.card)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_lg.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        text_color: Some(tokens.card_foreground),
        snap: false,
    }
}

/// Ghost card — no border, no shadow. Just a tinted surface.
pub fn ghost_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme) -> Style + 'a {
    move |_theme| Style {
        background: Some(Background::Color(with_alpha(tokens.card, 0.5))),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: tokens.radius_lg.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(tokens.card_foreground),
        snap: false,
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a styled card container.
pub fn styled<'a, Message: 'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(content)
        .padding(tokens.spacing.lg)
        .style(default_style(tokens))
}

/// Create an **elevated** card container.
pub fn elevated<'a, Message: 'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    container(content)
        .padding(tokens.spacing.lg)
        .style(elevated_style(tokens))
}

/// Create a card with **header**, **content**, and **footer** sections.
pub fn sectioned<'a, Message: 'a>(
    header: Option<Element<'a, Message, Theme>>,
    content: Element<'a, Message, Theme>,
    footer: Option<Element<'a, Message, Theme>>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    let mut col = Column::new().spacing(tokens.spacing.md);

    if let Some(h) = header {
        col = col.push(
            container(h).padding(Padding::default().bottom(tokens.spacing.sm)),
        );
    }

    col = col.push(content);

    if let Some(f) = footer {
        col = col.push(
            container(f).padding(Padding::default().top(tokens.spacing.sm)),
        );
    }

    container(col)
        .padding(tokens.spacing.lg)
        .style(default_style(tokens))
}
