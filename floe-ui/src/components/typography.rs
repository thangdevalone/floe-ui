//! Typography component — standardized text styles matching shadcn/ui.

use crate::theme::tokens::DesignTokens;
use iced::widget::text;
use iced::widget::Text;
use iced::Theme;

/// Create an H1 heading (Scroll-m-20 text-4xl font-extrabold tracking-tight lg:text-5xl)
pub fn h1<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(36) // 4xl approximation
        .color(tokens.foreground)
}

/// Create an H2 heading (Scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0)
pub fn h2<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(30) // 3xl approximation
        .color(tokens.foreground)
}

/// Create an H3 heading (Scroll-m-20 text-2xl font-semibold tracking-tight)
pub fn h3<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(24) // 2xl approximation
        .color(tokens.foreground)
}

/// Create an H4 heading (Scroll-m-20 text-xl font-semibold tracking-tight)
pub fn h4<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(20) // xl approximation
        .color(tokens.foreground)
}

/// Create a standard paragraph (Leading-7 [&:not(:first-child)]:mt-6)
pub fn p<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(16)
        .line_height(iced::widget::text::LineHeight::Relative(1.75))
        .color(tokens.foreground)
}

/// Create lead text (Text-xl text-muted-foreground)
pub fn lead<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(20)
        .color(tokens.muted_foreground)
}

/// Create large text (Text-lg font-semibold)
pub fn large<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(18)
        .color(tokens.foreground)
}

/// Create small text (Text-sm font-medium leading-none)
pub fn small<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(14)
        .color(tokens.foreground)
}

/// Create muted text (Text-sm text-muted-foreground)
pub fn muted<'a>(content: &str, tokens: &'a DesignTokens) -> Text<'a, Theme> {
    text(content.to_string())
        .size(14)
        .color(tokens.muted_foreground)
}
