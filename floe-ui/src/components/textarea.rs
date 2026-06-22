//! Textarea component — styled wrapper around Iced's `text_editor`.

use crate::theme::tokens::{with_alpha, DesignTokens};
use iced::widget::text_editor::{self, Status, Style};
use iced::widget::{text_editor as editor, TextEditor};
use iced::{Background, Border, Theme};
use iced::advanced::text::highlighter::PlainText;

// ── Style function ──────────────────────────────────────────────────────

/// Default textarea styling.
pub fn default_style<'a>(
    tokens: &'a DesignTokens,
) -> impl Fn(&Theme, Status) -> Style + 'a {
    move |_theme, status| {
        let base_border = Border {
            color: tokens.input,
            width: 1.0,
            radius: tokens.radius_md.into(),
        };

        let (bg, border) = match status {
            Status::Active => (tokens.background, base_border),
            Status::Hovered { .. } => (
                tokens.background,
                Border {
                    color: tokens.ring,
                    ..base_border
                },
            ),
            Status::Focused { .. } => (
                tokens.background,
                Border {
                    color: tokens.ring,
                    width: 2.0,
                    ..base_border
                },
            ),
            Status::Disabled => (
                with_alpha(tokens.muted, 0.5),
                Border {
                    color: with_alpha(tokens.input, 0.5),
                    ..base_border
                },
            ),
        };

        Style {
            background: Background::Color(bg),
            border,
            placeholder: tokens.muted_foreground,
            value: tokens.foreground,
            selection: with_alpha(tokens.primary, 0.2),
        }
    }
}

// ── Builder function ────────────────────────────────────────────────────

/// Create a styled textarea.
pub fn styled<'a, Message: Clone + 'a>(
    content: &'a text_editor::Content,
    on_action: impl Fn(text_editor::Action) -> Message + 'a,
    tokens: &'a DesignTokens,
) -> TextEditor<'a, PlainText, Message, Theme> {
    editor(content)
        .on_action(on_action)
        .padding(12)
        .style(default_style(tokens))
}
