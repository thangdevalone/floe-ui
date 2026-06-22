//! Design tokens — semantic color, spacing, and radius definitions.
//!
//! This module mirrors the CSS custom property system used by shadcn/ui,
//! but expressed as typed Rust structs. Every component in Floe UI reads
//! its colors, radii, and spacing from these tokens, making global theme
//! changes trivial.

use iced::Color;

/// Semantic design tokens that drive the entire visual system.
///
/// Token names are intentionally aligned with shadcn/ui's CSS variable
/// naming (`--background`, `--primary`, `--muted`, etc.) so that
/// designers familiar with the web library can transfer knowledge directly.
#[derive(Debug, Clone)]
pub struct DesignTokens {
    // ── Core semantic colors ────────────────────────────────────────
    /// App background color.
    pub background: Color,
    /// Default text / foreground color.
    pub foreground: Color,

    // ── Card ────────────────────────────────────────────────────────
    /// Card surface color.
    pub card: Color,
    /// Text color on cards.
    pub card_foreground: Color,

    // ── Popover ─────────────────────────────────────────────────────
    /// Popover / dropdown surface.
    pub popover: Color,
    /// Text color inside popovers.
    pub popover_foreground: Color,

    // ── Primary ─────────────────────────────────────────────────────
    /// Primary brand color (buttons, links, accents).
    pub primary: Color,
    /// Text on primary-colored surfaces.
    pub primary_foreground: Color,

    // ── Secondary ───────────────────────────────────────────────────
    /// Secondary / muted action color.
    pub secondary: Color,
    /// Text on secondary surfaces.
    pub secondary_foreground: Color,

    // ── Muted ───────────────────────────────────────────────────────
    /// Muted backgrounds (disabled states, subtle sections).
    pub muted: Color,
    /// Text on muted surfaces.
    pub muted_foreground: Color,

    // ── Accent ──────────────────────────────────────────────────────
    /// Accent highlight (hover states, selections).
    pub accent: Color,
    /// Text on accent surfaces.
    pub accent_foreground: Color,

    // ── Destructive ─────────────────────────────────────────────────
    /// Danger / destructive actions.
    pub destructive: Color,
    /// Text on destructive surfaces.
    pub destructive_foreground: Color,

    // ── Borders & inputs ────────────────────────────────────────────
    /// Default border color.
    pub border: Color,
    /// Input field border color.
    pub input: Color,
    /// Focus ring color.
    pub ring: Color,

    // ── Border radius scale ─────────────────────────────────────────
    pub radius_sm: f32,
    pub radius_md: f32,
    pub radius_lg: f32,
    pub radius_xl: f32,
    pub radius_full: f32,

    // ── Spacing scale ───────────────────────────────────────────────
    pub spacing: SpacingScale,
}

/// A fixed spacing scale (in logical pixels) for consistent layouts.
#[derive(Debug, Clone, Copy)]
pub struct SpacingScale {
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl Default for SpacingScale {
    fn default() -> Self {
        Self {
            xs: 4.0,
            sm: 8.0,
            md: 12.0,
            lg: 16.0,
            xl: 24.0,
            xxl: 32.0,
        }
    }
}

impl DesignTokens {
    /// Default border radius scale.
    pub fn default_radii() -> (f32, f32, f32, f32, f32) {
        (4.0, 6.0, 8.0, 12.0, 9999.0)
    }
}

// ── Color helpers ───────────────────────────────────────────────────────

/// Create a [`Color`] from an HSL triple.
///
/// - `h`: hue in degrees \[0, 360)
/// - `s`: saturation \[0.0, 1.0\]
/// - `l`: lightness \[0.0, 1.0\]
pub fn hsl(h: f32, s: f32, l: f32) -> Color {
    hsla(h, s, l, 1.0)
}

/// Create a [`Color`] from HSLA values.
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::from_rgba(r + m, g + m, b + m, a)
}

/// Apply an alpha value to an existing color.
pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}

/// Lighten a color by mixing it toward white.
pub fn lighten(color: Color, amount: f32) -> Color {
    Color {
        r: color.r + (1.0 - color.r) * amount,
        g: color.g + (1.0 - color.g) * amount,
        b: color.b + (1.0 - color.b) * amount,
        a: color.a,
    }
}

/// Darken a color by mixing it toward black.
pub fn darken(color: Color, amount: f32) -> Color {
    Color {
        r: color.r * (1.0 - amount),
        g: color.g * (1.0 - amount),
        b: color.b * (1.0 - amount),
        a: color.a,
    }
}
