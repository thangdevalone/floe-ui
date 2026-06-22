# Contributing to Floe UI

First off, thank you for considering contributing to Floe UI! It's people like you that make this library a great tool for the Iced community.

## 🚀 How to Contribute

### 1. Reporting Bugs
If you find a bug, please create an issue on GitHub with:
- A clear title and description of the issue.
- The version of `iced` and `floe-ui` you are using.
- A minimal reproducible example if possible.
- Screenshots if it's a visual bug.

### 2. Suggesting Enhancements
We welcome ideas for new components or improvements to existing ones! Open an issue describing:
- The component or feature you want.
- Why it would be useful for the community.
- Potential API design or visual inspiration (e.g., links to Shadcn UI, Radix UI).

### 3. Submitting Pull Requests
1. Fork the repository and create your branch from `main`.
2. Make your changes in the `floe-ui` crate.
3. If you add a new component, please also add it to the `web-demo` documentation site so people can see how it looks.
4. Run `cargo fmt` and `cargo clippy` to ensure code quality.
5. Create a Pull Request with a clear description of the changes.

## 🛠️ Development Setup

1. Clone the repository:
```bash
git clone https://github.com/thangdevalone/floe-ui.git
cd floe-ui
```

2. Run the Web Demo (Next.js) to view the documentation locally:
```bash
cd web-demo
npm install
npm run dev
```

3. Work on the Rust components in the `floe-ui/src/components` directory. 

## 🎨 Component Guidelines

- **Token-Driven:** Always use the `DesignTokens` struct for colors, spacing, and border radii instead of hardcoding values.
- **Micro-Animations:** Try to include smooth hover and active state transitions.
- **Copy-Paste Friendly:** Keep component code as self-contained as possible, as users will be downloading the raw `.rs` files into their projects.

## 📝 License

By contributing to Floe UI, you agree that your contributions will be licensed under its MIT License.
