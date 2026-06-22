//! Beautifully styled buttons with multiple variants and sizes.
//!
//! Mirrors shadcn/ui's Button component with variants:
//! `Default`, `Secondary`, `Outline`, `Ghost`, `Destructive`, `Link`.

use crate::theme::tokens::{darken, with_alpha, DesignTokens};
use iced::widget::button::{Status, Style};
use iced::widget::{button, text, Button};
use iced::{Background, Border, Color, Padding, Shadow, Theme, Vector};

// ── Size presets ────────────────────────────────────────────────────────

/// Button size preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    /// Small: compact padding, smaller text.
    Sm,
    /// Default: standard padding and text.
    Default,
    /// Large: generous padding, larger text.
    Lg,
    /// Icon-only: square aspect ratio.
    Icon,
}

impl ButtonSize {
    /// Returns padding for this size.
    pub fn padding(self) -> Padding {
        match self {
            Self::Sm => Padding::from([6.0, 12.0]),
            Self::Default => Padding::from([8.0, 16.0]),
            Self::Lg => Padding::from([10.0, 32.0]),
            Self::Icon => Padding::from([8.0, 8.0]),
        }
    }

    /// Text size (in pixels) for this button size.
    pub fn text_size(self) -> f32 {
        match self {
            Self::Sm => 13.0,
            Self::Default => 14.0,
            Self::Lg => 16.0,
            Self::Icon => 14.0,
        }
    }
}

// ── Style helper ────────────────────────────────────────────────────────

fn base_style(
    bg: Option<Color>,
    fg: Color,
    radius: f32,
    border_color: Option<Color>,
    shadow: bool,
) -> Style {
    Style {
        background: bg.map(Background::Color),
        text_color: fg,
        border: Border {
            color: border_color.unwrap_or(Color::TRANSPARENT),
            width: if border_color.is_some() { 1.0 } else { 0.0 },
            radius: radius.into(),
        },
        shadow: if shadow {
            Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            }
        } else {
            Shadow::default()
        },
        snap: false,
    }
}

// ── Variant style functions ─────────────────────────────────────────────

/// Returns a style function for the **primary / default** button variant.
pub fn primary_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| match status {
        Status::Active => base_style(
            Some(tokens.primary),
            tokens.primary_foreground,
            tokens.radius_md,
            None,
            true,
        ),
        Status::Hovered => base_style(
            Some(with_alpha(tokens.primary, 0.9)),
            tokens.primary_foreground,
            tokens.radius_md,
            None,
            true,
        ),
        Status::Pressed => base_style(
            Some(darken(tokens.primary, 0.1)),
            tokens.primary_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Disabled => base_style(
            Some(with_alpha(tokens.primary, 0.5)),
            with_alpha(tokens.primary_foreground, 0.5),
            tokens.radius_md,
            None,
            false,
        ),
    }
}

/// **Secondary** variant — muted background.
pub fn secondary_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| match status {
        Status::Active => base_style(
            Some(tokens.secondary),
            tokens.secondary_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Hovered => base_style(
            Some(with_alpha(tokens.secondary, 0.8)),
            tokens.secondary_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Pressed => base_style(
            Some(darken(tokens.secondary, 0.05)),
            tokens.secondary_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Disabled => base_style(
            Some(with_alpha(tokens.secondary, 0.5)),
            with_alpha(tokens.secondary_foreground, 0.5),
            tokens.radius_md,
            None,
            false,
        ),
    }
}

/// **Outline** variant — transparent background with border.
pub fn outline_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| match status {
        Status::Active => base_style(
            Some(Color::TRANSPARENT),
            tokens.foreground,
            tokens.radius_md,
            Some(tokens.input),
            false,
        ),
        Status::Hovered => base_style(
            Some(tokens.accent),
            tokens.accent_foreground,
            tokens.radius_md,
            Some(tokens.input),
            false,
        ),
        Status::Pressed => base_style(
            Some(darken(tokens.accent, 0.05)),
            tokens.accent_foreground,
            tokens.radius_md,
            Some(tokens.input),
            false,
        ),
        Status::Disabled => base_style(
            Some(Color::TRANSPARENT),
            with_alpha(tokens.foreground, 0.5),
            tokens.radius_md,
            Some(with_alpha(tokens.input, 0.5)),
            false,
        ),
    }
}

/// **Ghost** variant — no background, no border, subtle hover.
pub fn ghost_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| match status {
        Status::Active => base_style(
            Some(Color::TRANSPARENT),
            tokens.foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Hovered => base_style(
            Some(tokens.accent),
            tokens.accent_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Pressed => base_style(
            Some(darken(tokens.accent, 0.05)),
            tokens.accent_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Disabled => base_style(
            Some(Color::TRANSPARENT),
            with_alpha(tokens.foreground, 0.5),
            tokens.radius_md,
            None,
            false,
        ),
    }
}

/// **Destructive** variant — danger / delete actions.
pub fn destructive_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| match status {
        Status::Active => base_style(
            Some(tokens.destructive),
            tokens.destructive_foreground,
            tokens.radius_md,
            None,
            true,
        ),
        Status::Hovered => base_style(
            Some(with_alpha(tokens.destructive, 0.9)),
            tokens.destructive_foreground,
            tokens.radius_md,
            None,
            true,
        ),
        Status::Pressed => base_style(
            Some(darken(tokens.destructive, 0.1)),
            tokens.destructive_foreground,
            tokens.radius_md,
            None,
            false,
        ),
        Status::Disabled => base_style(
            Some(with_alpha(tokens.destructive, 0.5)),
            with_alpha(tokens.destructive_foreground, 0.5),
            tokens.radius_md,
            None,
            false,
        ),
    }
}

/// **Link** variant — looks like a hyperlink.
pub fn link_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let fg = match status {
            Status::Active => tokens.primary,
            Status::Hovered => darken(tokens.primary, 0.1),
            Status::Pressed => darken(tokens.primary, 0.2),
            Status::Disabled => with_alpha(tokens.primary, 0.5),
        };
        base_style(Some(Color::TRANSPARENT), fg, 0.0, None, false)
    }
}

// ── Builder functions ───────────────────────────────────────────────────

/// Create a **primary** (default) button.
pub fn primary<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(ButtonSize::Default.text_size()))
        .padding(ButtonSize::Default.padding())
        .style(primary_style(tokens))
}

/// Create a **secondary** button.
pub fn secondary<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(ButtonSize::Default.text_size()))
        .padding(ButtonSize::Default.padding())
        .style(secondary_style(tokens))
}

/// Create an **outline** button.
pub fn outline<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(ButtonSize::Default.text_size()))
        .padding(ButtonSize::Default.padding())
        .style(outline_style(tokens))
}

/// Create a **ghost** button.
pub fn ghost<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(ButtonSize::Default.text_size()))
        .padding(ButtonSize::Default.padding())
        .style(ghost_style(tokens))
}

/// Create a **destructive** button.
pub fn destructive<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(ButtonSize::Default.text_size()))
        .padding(ButtonSize::Default.padding())
        .style(destructive_style(tokens))
}

/// Create a **link** button.
pub fn link<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(ButtonSize::Default.text_size()))
        .padding(ButtonSize::Default.padding())
        .style(link_style(tokens))
}

/// Create a **primary button with custom size**.
pub fn primary_sized<'a, Message: Clone + 'a>(
    label: &str,
    tokens: &'a DesignTokens,
    size: ButtonSize,
) -> Button<'a, Message, Theme> {
    button(text(label.to_string()).size(size.text_size()))
        .padding(size.padding())
        .style(primary_style(tokens))
}
