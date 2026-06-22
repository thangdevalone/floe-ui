//! Alert component — info, warning, error, and success alert boxes.
//!
//! Mirrors shadcn/ui's Alert component with icon + title + description.

use crate::components::icon::{self, IconName};
use crate::theme::tokens::DesignTokens;
use iced::widget::container::Style;
use iced::widget::{column, container, row, text, Container};
use iced::{Background, Border, Color, Shadow, Theme};

/// Alert variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertVariant {
    /// Default / informational.
    Default,
    /// Destructive / error.
    Destructive,
    /// Success.
    Success,
    /// Warning.
    Warning,
}

// ── Style functions ─────────────────────────────────────────────────────

/// Style for an alert based on variant.
pub fn alert_style<'a>(
    tokens: &'a DesignTokens,
    variant: AlertVariant,
) -> impl Fn(&Theme) -> Style + 'a {
    let success_color = Color::from_rgb(0.29, 0.78, 0.47);
    let warning_color = Color::from_rgb(0.90, 0.75, 0.30);

    move |_theme| {
        let (border_color, fg) = match variant {
            AlertVariant::Default => (
                tokens.border,
                tokens.foreground,
            ),
            AlertVariant::Destructive => (
                tokens.destructive,
                tokens.destructive,
            ),
            AlertVariant::Success => (
                success_color,
                success_color,
            ),
            AlertVariant::Warning => (
                warning_color,
                warning_color,
            ),
        };

        // shadcn/ui uses transparent background for alerts, relying on the colored
        // border and text for differentiation to avoid muddy colors in dark mode.
        Style {
            background: Some(Background::Color(Color::TRANSPARENT)),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: tokens.radius_lg.into(),
            },
            shadow: Shadow::default(),
            text_color: Some(fg),
            snap: false,
        }
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create an alert with icon, title, and description.
pub fn styled<'a, Message: 'a>(
    icon: IconName,
    title: &str,
    description: &str,
    variant: AlertVariant,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    let content = row![
        icon::view(icon).size(18),
        column![
            text(title.to_string()).size(14),
            text(description.to_string())
                // Always use foreground matching the alert variant for best contrast
                // when background is transparent.
                .size(13),
        ]
        .spacing(4),
    ]
    .spacing(12);

    container(content)
        .padding([12, 16])
        .style(alert_style(tokens, variant))
}

/// Create a default/info alert.
pub fn info<'a, Message: 'a>(
    title: &str,
    description: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    styled(IconName::Info, title, description, AlertVariant::Default, tokens)
}

/// Create a destructive/error alert.
pub fn error<'a, Message: 'a>(
    title: &str,
    description: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    styled(IconName::TriangleAlert, title, description, AlertVariant::Destructive, tokens)
}

/// Create a success alert.
pub fn success<'a, Message: 'a>(
    title: &str,
    description: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    styled(IconName::CircleCheck, title, description, AlertVariant::Success, tokens)
}

/// Create a warning alert.
pub fn warning<'a, Message: 'a>(
    title: &str,
    description: &str,
    tokens: &'a DesignTokens,
) -> Container<'a, Message, Theme> {
    styled(IconName::TriangleAlert, title, description, AlertVariant::Warning, tokens)
}
