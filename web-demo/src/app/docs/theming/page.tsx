import { CodeBlock } from "@/components/code-block";
import { ApiTable, type ApiProp } from "@/components/api-table";

const tokenProps: ApiProp[] = [
  { name: "background", type: "Color", description: "App background color" },
  { name: "foreground", type: "Color", description: "Default text / foreground color" },
  { name: "card", type: "Color", description: "Card surface color" },
  { name: "card_foreground", type: "Color", description: "Text color on cards" },
  { name: "popover", type: "Color", description: "Popover / dropdown surface" },
  { name: "popover_foreground", type: "Color", description: "Text inside popovers" },
  { name: "primary", type: "Color", description: "Primary brand color (buttons, links)" },
  { name: "primary_foreground", type: "Color", description: "Text on primary surfaces" },
  { name: "secondary", type: "Color", description: "Secondary / muted action color" },
  { name: "secondary_foreground", type: "Color", description: "Text on secondary surfaces" },
  { name: "muted", type: "Color", description: "Muted backgrounds (disabled, subtle)" },
  { name: "muted_foreground", type: "Color", description: "Text on muted surfaces" },
  { name: "accent", type: "Color", description: "Accent highlight (hovers, selections)" },
  { name: "accent_foreground", type: "Color", description: "Text on accent surfaces" },
  { name: "destructive", type: "Color", description: "Danger / destructive actions" },
  { name: "destructive_foreground", type: "Color", description: "Text on destructive surfaces" },
  { name: "border", type: "Color", description: "Default border color" },
  { name: "input", type: "Color", description: "Input field border color" },
  { name: "ring", type: "Color", description: "Focus ring color" },
];

const radiusProps: ApiProp[] = [
  { name: "radius_sm", type: "f32", default: "4.0", description: "Small radius — inner elements" },
  { name: "radius_md", type: "f32", default: "6.0", description: "Medium radius — buttons, inputs" },
  { name: "radius_lg", type: "f32", default: "8.0", description: "Large radius — cards, modals" },
  { name: "radius_xl", type: "f32", default: "12.0", description: "Extra large radius — hero sections" },
  { name: "radius_full", type: "f32", default: "9999.0", description: "Full radius — pills, avatars" },
];

const spacingProps: ApiProp[] = [
  { name: "xs", type: "f32", default: "4.0", description: "Tightest spacing" },
  { name: "sm", type: "f32", default: "8.0", description: "Small spacing" },
  { name: "md", type: "f32", default: "12.0", description: "Medium spacing" },
  { name: "lg", type: "f32", default: "16.0", description: "Large spacing — default padding" },
  { name: "xl", type: "f32", default: "24.0", description: "Extra large spacing" },
  { name: "xxl", type: "f32", default: "32.0", description: "Widest spacing" },
];

export default function ThemingPage() {
  return (
    <div className="space-y-10">
      <div className="space-y-2">
        <h1 className="scroll-m-20 text-4xl font-bold tracking-tight">Theming</h1>
        <p className="text-lg text-muted-foreground">
          Customize every aspect of Floe UI using the design token system.
        </p>
      </div>

      {/* Overview */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">How It Works</h2>
        <p className="leading-7">
          Floe UI uses a <strong>DesignTokens</strong> struct that mirrors the CSS custom property system
          from shadcn/ui (<code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">--background</code>,{" "}
          <code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">--primary</code>, etc.).
          Every component reads its colors, radii, and spacing from these tokens — so changing one token
          updates every component that uses it.
        </p>
        <CodeBlock
          code={`use floe_ui::prelude::*;

// Use a built-in theme
let theme = FloeTheme::zinc_dark();
let tokens = &theme.tokens;

// All components automatically use these tokens
let btn = button::primary("Click me", tokens);
let card = card::styled(text("Hello!"), tokens);`}
          filename="theming.rs"
        />
      </section>

      {/* Token Reference */}
      <section className="space-y-6">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Token Reference</h2>

        <ApiTable title="Semantic Colors" props={tokenProps} />

        <ApiTable title="Border Radius Scale" props={radiusProps} />

        <ApiTable title="Spacing Scale" props={spacingProps} />
      </section>

      {/* Custom Theme */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Creating a Custom Theme</h2>
        <p className="leading-7">
          You can create a fully custom theme by constructing <strong>DesignTokens</strong> with your own colors.
          Use the <code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">hsl()</code> and{" "}
          <code className="font-mono text-xs bg-muted/50 px-1.5 py-0.5 rounded">hsla()</code> helpers for easy color
          definition.
        </p>
        <CodeBlock
          code={`use floe_ui::prelude::*;

let my_tokens = DesignTokens {
    background: hsl(222.0, 0.84, 0.05),    // Deep navy
    foreground: hsl(210.0, 0.40, 0.98),     // Near white
    primary: hsl(217.0, 0.91, 0.60),        // Bright blue
    primary_foreground: hsl(0.0, 0.0, 1.0), // White

    // ... fill in all other tokens ...

    // Use default radii
    radius_sm: 4.0,
    radius_md: 6.0,
    radius_lg: 8.0,
    radius_xl: 12.0,
    radius_full: 9999.0,

    spacing: SpacingScale::default(),
};`}
          filename="custom_theme.rs"
        />
      </section>

      {/* Color Helpers */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Color Helper Functions</h2>
        <p className="leading-7 mb-4">
          Floe UI provides utility functions for working with colors:
        </p>

        <div className="rounded-xl border border-border overflow-hidden">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-border bg-muted/30">
                <th className="text-left px-4 py-3 font-medium text-muted-foreground">Function</th>
                <th className="text-left px-4 py-3 font-medium text-muted-foreground">Description</th>
              </tr>
            </thead>
            <tbody>
              <tr className="border-b border-border/50">
                <td className="px-4 py-3 font-mono text-[13px] text-yellow-300">hsl(h, s, l)</td>
                <td className="px-4 py-3 text-muted-foreground">Create a Color from HSL values (hue 0-360, saturation 0-1, lightness 0-1)</td>
              </tr>
              <tr className="border-b border-border/50">
                <td className="px-4 py-3 font-mono text-[13px] text-yellow-300">hsla(h, s, l, a)</td>
                <td className="px-4 py-3 text-muted-foreground">Create a Color from HSLA values with alpha</td>
              </tr>
              <tr className="border-b border-border/50">
                <td className="px-4 py-3 font-mono text-[13px] text-yellow-300">with_alpha(color, alpha)</td>
                <td className="px-4 py-3 text-muted-foreground">Apply an alpha value (0.0 – 1.0) to an existing color</td>
              </tr>
              <tr className="border-b border-border/50">
                <td className="px-4 py-3 font-mono text-[13px] text-yellow-300">darken(color, amount)</td>
                <td className="px-4 py-3 text-muted-foreground">Darken a color by mixing toward black</td>
              </tr>
              <tr className="border-b border-border/50 last:border-0">
                <td className="px-4 py-3 font-mono text-[13px] text-yellow-300">lighten(color, amount)</td>
                <td className="px-4 py-3 text-muted-foreground">Lighten a color by mixing toward white</td>
              </tr>
            </tbody>
          </table>
        </div>
      </section>

      {/* Color Preview */}
      <section className="space-y-4">
        <h2 className="text-xl font-semibold tracking-tight border-b border-border pb-2">Current Theme Preview</h2>
        <p className="text-sm text-muted-foreground">The Zinc Dark theme used on this site:</p>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
          {[
            { name: "Background", color: "bg-background", border: true },
            { name: "Foreground", color: "bg-foreground" },
            { name: "Primary", color: "bg-primary" },
            { name: "Secondary", color: "bg-secondary" },
            { name: "Muted", color: "bg-muted" },
            { name: "Accent", color: "bg-accent" },
            { name: "Destructive", color: "bg-destructive" },
            { name: "Border", color: "bg-border" },
          ].map((item) => (
            <div key={item.name} className="flex flex-col items-center gap-2">
              <div
                className={`w-full h-16 rounded-lg ${item.color} ${item.border ? "border border-border" : ""}`}
              />
              <span className="text-xs text-muted-foreground">{item.name}</span>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}
