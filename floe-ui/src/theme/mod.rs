//! Floe UI theme system.
//!
//! [`FloeTheme`] wraps Iced's built-in [`Theme`] while carrying a full
//! set of [`DesignTokens`]. Every Floe UI component reads its appearance
//! from these tokens, making it trivial to swap between palettes or
//! create entirely custom themes.

pub mod palette;
pub mod tokens;

pub use palette::*;
pub use tokens::{darken, hsl, hsla, lighten, with_alpha, DesignTokens, SpacingScale};

use iced::theme::Palette;
use iced::Color;
use iced::Theme;

/// The Floe UI theme.
///
/// Combines Iced's built-in [`Theme`] with a full set of semantic
/// [`DesignTokens`] so that every component can read colours, radii,
/// and spacing from a single source of truth.
#[derive(Debug, Clone)]
pub struct FloeTheme {
    /// Semantic design tokens.
    pub tokens: DesignTokens,
    /// The underlying Iced theme (used when calling `.style()` on
    /// native widgets).
    pub inner: Theme,
}

impl FloeTheme {
    /// Create a theme from a name and a set of tokens.
    ///
    /// This builds an Iced `Palette` from the tokens and lets Iced
    /// auto-generate the `Extended` palette via `Theme::custom`.
    pub fn from_tokens(name: &str, tokens: DesignTokens) -> Self {
        let iced_palette = Palette {
            background: tokens.background,
            text: tokens.foreground,
            primary: tokens.primary,
            success: Color::from_rgb(0.29, 0.78, 0.47),
            warning: Color::from_rgb(0.90, 0.75, 0.30),
            danger: tokens.destructive,
        };

        // `Theme::custom` auto-generates the Extended palette from
        // the base Palette using `Extended::generate`.
        let inner = Theme::custom(name.to_string(), iced_palette);

        Self { tokens, inner }
    }

    // ── Preset constructors ─────────────────────────────────────────

    /// Zinc Dark — the default Floe UI theme.
    pub fn zinc_dark() -> Self {
        Self::from_tokens("Floe Zinc Dark", zinc_dark())
    }

    /// Zinc Light.
    pub fn zinc_light() -> Self {
        Self::from_tokens("Floe Zinc Light", zinc_light())
    }

    /// Slate Dark.
    pub fn custom(tokens: DesignTokens) -> Self {
        Self::from_tokens("Floe Custom", tokens)
    }

    /// Get a reference to the underlying Iced theme.
    pub fn iced_theme(&self) -> &Theme {
        &self.inner
    }
}
