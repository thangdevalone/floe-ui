//! Slider component — styled wrapper around Iced's `slider`.

use crate::theme::tokens::{with_alpha, darken, DesignTokens};
use iced::widget::slider::{self, HandleShape, Status, Style};
use iced::widget::{slider as slider_widget, Slider};
use iced::{Background, Border, Color, Theme};

// ── Style functions ─────────────────────────────────────────────────────

/// Default slider styling — primary-colored track and handle.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let (rail_bg, handle_bg, handle_border) = match status {
            Status::Active => (
                tokens.secondary,
                tokens.primary,
                tokens.primary,
            ),
            Status::Hovered => (
                tokens.secondary,
                with_alpha(tokens.primary, 0.9),
                tokens.ring,
            ),
            Status::Dragged => (
                tokens.secondary,
                darken(tokens.primary, 0.1),
                tokens.ring,
            ),
        };

        Style {
            rail: slider::Rail {
                backgrounds: (
                    Background::Color(tokens.primary),
                    Background::Color(rail_bg),
                ),
                width: 4.0,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: tokens.radius_full.into(),
                },
            },
            handle: slider::Handle {
                shape: HandleShape::Circle { radius: 8.0 },
                background: Background::Color(handle_bg),
                border_width: 2.0,
                border_color: handle_border,
            },
        }
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Create a styled slider.
pub fn styled<'a, Message: Clone + 'a>(
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    on_change: impl Fn(f32) -> Message + 'a,
    tokens: &'a DesignTokens,
) -> Slider<'a, f32, Message, Theme> {
    slider_widget(range, value, on_change)
        .step(0.01)
        .style(default_style(tokens))
}
