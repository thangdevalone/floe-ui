import re

def to_pascal_case(kebab):
    return "".join(word.capitalize() for word in kebab.split("-"))

with open("c:\\Project\\Floe UI\\floe-ui\\fonts\\lucide.css", "r", encoding="utf-8") as f:
    css = f.read()

matches = re.findall(r'\.icon-(.*?):before \{ content: "(.*?)"; \}', css)

enum_variants = []
match_arms = []

for name, code in matches:
    variant = to_pascal_case(name)
    # prevent invalid rust identifiers
    if variant[0].isdigit():
        variant = "N" + variant
    # handle python unicode escapes
    code = code.replace("\\", "\\u{") + "}"
    enum_variants.append(f"    {variant},")
    match_arms.append(f'            Self::{variant} => "{code}",')

rs_code = f"""//! Icon component — wrapper around Iced's `text` widget using Lucide font.
//!
//! To use this, you MUST register the Lucide font in your application:
//! ```rust
//! iced::application(..)
//!     .font(floe_ui::components::icon::LUCIDE_FONT_BYTES)
//!     // ...
//! ```

use iced::widget::{{text, Text}};
use iced::{{Font, Theme}};

/// The raw bytes of the Lucide TTF font.
pub const LUCIDE_FONT_BYTES: &[u8] = include_bytes!("../../fonts/lucide.ttf");

/// The Iced `Font` definition for Lucide.
pub const LUCIDE_FONT: Font = Font::with_name("lucide");

/// Pre-defined icons from the Lucide set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {{
{chr(10).join(enum_variants)}
}}

impl IconName {{
    /// Returns the unicode character for this icon.
    pub fn to_unicode(self) -> &'static str {{
        match self {{
{chr(10).join(match_arms)}
        }}
    }}
    
    pub fn all() -> &'static [Self] {{
        &[
{chr(10).join(["            Self::" + (to_pascal_case(n) if not to_pascal_case(n)[0].isdigit() else "N" + to_pascal_case(n)) + "," for n, _ in matches])}
        ]
    }}
}}

/// Create a text widget displaying the requested icon.
pub fn view<'a>(name: IconName) -> Text<'a, Theme> {{
    text(name.to_unicode())
        .font(LUCIDE_FONT)
        .line_height(iced::widget::text::LineHeight::Relative(1.0))
        .shaping(iced::widget::text::Shaping::Basic)
}}
"""

with open("c:\\Project\\Floe UI\\floe-ui\\src\\components\\icon.rs", "w", encoding="utf-8") as f:
    f.write(rs_code)
