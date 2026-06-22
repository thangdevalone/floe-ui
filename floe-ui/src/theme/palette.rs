//! Predefined color palettes for Floe UI.
//!
//! Currently focused on the core `Zinc` palette (dark/light).
//! You can always create your own `DesignTokens` for custom themes.

use super::tokens::{hsl, DesignTokens, SpacingScale};

/// Zinc Dark — the default shadcn/ui dark palette.
///
/// Neutral, clean, and modern. Works beautifully for developer tools
/// and professional applications.
pub fn zinc_dark() -> DesignTokens {
    let (radius_sm, radius_md, radius_lg, radius_xl, radius_full) =
        DesignTokens::default_radii();

    DesignTokens {
        // Core
        background: hsl(240.0, 0.10, 0.039),
        foreground: hsl(0.0, 0.0, 0.98),

        // Card
        card: hsl(240.0, 0.10, 0.039),
        card_foreground: hsl(0.0, 0.0, 0.98),

        // Popover
        popover: hsl(240.0, 0.10, 0.039),
        popover_foreground: hsl(0.0, 0.0, 0.98),

        // Primary
        primary: hsl(0.0, 0.0, 0.98),
        primary_foreground: hsl(240.0, 0.059, 0.10),

        // Secondary
        secondary: hsl(240.0, 0.037, 0.159),
        secondary_foreground: hsl(0.0, 0.0, 0.98),

        // Muted
        muted: hsl(240.0, 0.037, 0.159),
        muted_foreground: hsl(240.0, 0.05, 0.649),

        // Accent
        accent: hsl(240.0, 0.037, 0.159),
        accent_foreground: hsl(0.0, 0.0, 0.98),

        // Destructive
        destructive: hsl(0.0, 0.75, 0.55),
        destructive_foreground: hsl(0.0, 0.0, 0.98),

        // Borders
        border: hsl(240.0, 0.037, 0.159),
        input: hsl(240.0, 0.037, 0.159),
        ring: hsl(240.0, 0.049, 0.839),

        // Radii
        radius_sm,
        radius_md,
        radius_lg,
        radius_xl,
        radius_full,

        // Spacing
        spacing: SpacingScale::default(),
    }
}

/// Zinc Light — the default shadcn/ui light palette.
pub fn zinc_light() -> DesignTokens {
    let (radius_sm, radius_md, radius_lg, radius_xl, radius_full) =
        DesignTokens::default_radii();

    DesignTokens {
        background: hsl(0.0, 0.0, 1.0),
        foreground: hsl(240.0, 0.10, 0.039),

        card: hsl(0.0, 0.0, 1.0),
        card_foreground: hsl(240.0, 0.10, 0.039),

        popover: hsl(0.0, 0.0, 1.0),
        popover_foreground: hsl(240.0, 0.10, 0.039),

        primary: hsl(240.0, 0.059, 0.10),
        primary_foreground: hsl(0.0, 0.0, 0.98),

        secondary: hsl(240.0, 0.048, 0.959),
        secondary_foreground: hsl(240.0, 0.059, 0.10),

        muted: hsl(240.0, 0.048, 0.959),
        muted_foreground: hsl(240.0, 0.038, 0.461),

        accent: hsl(240.0, 0.048, 0.959),
        accent_foreground: hsl(240.0, 0.059, 0.10),

        destructive: hsl(0.0, 0.842, 0.602),
        destructive_foreground: hsl(0.0, 0.0, 0.98),

        border: hsl(240.0, 0.059, 0.90),
        input: hsl(240.0, 0.059, 0.90),
        ring: hsl(240.0, 0.10, 0.039),

        radius_sm,
        radius_md,
        radius_lg,
        radius_xl,
        radius_full,

        spacing: SpacingScale::default(),
    }
}
