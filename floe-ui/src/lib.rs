//! # Floe UI
//!
//! A **shadcn/ui-inspired** component library for the
//! [Iced](https://iced.rs) GUI framework.
//!
//! Floe UI provides beautifully styled, token-driven components that
//! you own and can customise freely — just like shadcn/ui does for React.
//!
//! ## Quick Start
//!
//! ```no_run
//! use floe_ui::prelude::*;
//! use floe_ui::components::{button, card, input, badge};
//!
//! let theme = FloeTheme::zinc_dark();
//! let tokens = &theme.tokens;
//!
//! // Create a primary button
//! let btn = button::primary("Click me", tokens);
//!
//! // Create a styled input
//! let field = input::styled("Email…", "", tokens);
//!
//! // Create a card
//! let card = card::styled(iced::widget::text("Hello!"), tokens);
//! ```

pub mod components;
pub mod theme;

/// Prelude — import everything you need with `use floe_ui::prelude::*`.
pub mod prelude {
    pub use crate::theme::{FloeTheme, DesignTokens, SpacingScale};
    pub use crate::theme::{hsl, hsla, with_alpha, darken, lighten};
}
