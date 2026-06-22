<div align="center">
  <img src="https://raw.githubusercontent.com/thangdevalone/floe-ui/main/web-demo/public/logo.png" alt="Floe UI Logo" width="120" />
  <h1>Floe UI</h1>
  <p>Beautifully styled, token-driven components for the <a href="https://github.com/iced-rs/iced">Iced</a> GUI framework.</p>

  <a href="https://crates.io/crates/floe-ui"><img src="https://img.shields.io/crates/v/floe-ui.svg" alt="Crates.io" /></a>
  <a href="https://github.com/thangdevalone/floe-ui/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License: MIT" /></a>
</div>

## Philosophy

**Floe UI** is heavily inspired by **shadcn/ui**. It is *not* a traditional crate dependency where you import pre-compiled black-box widgets.

Instead, Floe UI gives you a CLI that downloads the raw source code of the components directly into your project. You own the code. You can customize the styling, logic, and layout exactly how you want.

## Quick Start

### 1. Install the CLI
Install the official Floe CLI globally via Cargo:
```bash
cargo install floe-ui
```

### 2. Initialize your project
Run the init command inside your Iced project (where your `Cargo.toml` is). This will download the base theme system and tokens into `src/floe_ui`.
```bash
floe-ui init
```

### 3. Add Components
Add only the components you need. The CLI will download the raw `.rs` source code straight into your project's `src/floe_ui/components` folder.
```bash
floe-ui add button
floe-ui add input
floe-ui add card
```

### 4. Use them in your app
Import the prelude and start building your GUI.
```rust
use floe_ui::prelude::*;
use floe_ui::components::{button, input};

// ... inside your view function
let theme = FloeTheme::zinc_dark();
let tokens = &theme.tokens;

let btn = button::primary("Click Me", tokens)
    .on_press(Message::Clicked);
```

## Features

- **Copy-Paste Philosophy**: You own the code. Fully customizable.
- **Token-Driven**: Built around `DesignTokens` (colors, spacing, radii) allowing for easy theming.
- **Dark & Light Modes**: Seamless switching via the provided palette system.
- **Beautiful Defaults**: Carefully crafted aesthetics, micro-animations, and smooth transitions out-of-the-box.
- **Icons Included**: Bundled with a wrapper for Lucide icons.

## Requirements

Floe UI components are built for `iced = "0.14"` (with the `advanced` feature enabled).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
