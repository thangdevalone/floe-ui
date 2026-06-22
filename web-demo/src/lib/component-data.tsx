import React from "react";
import type { ApiProp } from "@/components/api-table";

// ── Types ──────────────────────────────────────────────────────────────

export type ComponentCategory = "Form" | "Layout" | "Data Display" | "Feedback" | "Navigation";

export interface VariantDemo {
  label: string;
  element: React.ReactNode;
}

export interface ComponentData {
  name: string;
  slug: string;
  description: string;
  category: ComponentCategory;
  preview: React.ReactNode;
  variants?: VariantDemo[];
  code: string;
  apiProps: ApiProp[];
  styleFunctions: string[];
  builderFunctions: string[];
}

// ── Shared Demo Styles ─────────────────────────────────────────────────

const btnBase = "h-9 px-4 py-2 rounded-md text-sm font-medium transition-colors";
const inputBase = "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring";

// ── Component Registry ─────────────────────────────────────────────────

export const componentsData: Record<string, ComponentData> = {
  alert: {
    name: "Alert",
    slug: "alert",
    description: "Displays a callout with icon, title, and description for important messages.",
    category: "Feedback",
    preview: (
      <div className="flex flex-col gap-4 w-full max-w-md">
        <div className="rounded-lg border border-border p-4 flex gap-3">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="mt-0.5"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>
          <div className="flex flex-col gap-1"><p className="text-sm font-medium">Heads up!</p><p className="text-sm text-muted-foreground">You can use Floe UI components in any Iced app.</p></div>
        </div>
        <div className="rounded-lg border border-red-500/50 p-4 flex gap-3 text-red-500">
          <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="mt-0.5"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg>
          <div className="flex flex-col gap-1"><p className="text-sm font-medium">Error</p><p className="text-sm opacity-80">Your session has expired. Please log in again.</p></div>
        </div>
      </div>
    ),
    variants: [
      { label: "Default", element: <div className="rounded-lg border border-border p-4 flex gap-3 w-full"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg><div className="flex flex-col gap-1"><p className="text-sm font-medium">Information</p><p className="text-sm text-muted-foreground">This is a default alert.</p></div></div> },
      { label: "Destructive", element: <div className="rounded-lg border border-red-500/50 text-red-500 p-4 flex gap-3 w-full"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg><div className="flex flex-col gap-1"><p className="text-sm font-medium">Error</p><p className="text-sm opacity-80">Something went wrong.</p></div></div> },
      { label: "Success", element: <div className="rounded-lg border border-emerald-500/50 text-emerald-500 p-4 flex gap-3 w-full"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><path d="m9 11 3 3L22 4"/></svg><div className="flex flex-col gap-1"><p className="text-sm font-medium">Success</p><p className="text-sm opacity-80">Your changes have been saved.</p></div></div> },
      { label: "Warning", element: <div className="rounded-lg border border-yellow-500/50 text-yellow-500 p-4 flex gap-3 w-full"><svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><path d="M12 9v4"/><path d="M12 17h.01"/></svg><div className="flex flex-col gap-1"><p className="text-sm font-medium">Warning</p><p className="text-sm opacity-80">Your account is about to expire.</p></div></div> },
    ],
    code: `use floe_ui::components::alert::{self, AlertVariant};
use floe_ui::components::icon::IconName;

// Default info alert
let info = alert::info("Heads up!", "You can use Floe UI components.", &tokens);

// Error alert
let error = alert::error("Error", "Session expired.", &tokens);

// Success alert
let success = alert::success("Success", "Changes saved.", &tokens);

// Warning alert
let warning = alert::warning("Warning", "Account expiring.", &tokens);

// Custom alert with specific icon and variant
let custom = alert::styled(
    IconName::Info,
    "Custom Title",
    "Custom description text.",
    AlertVariant::Default,
    &tokens,
);`,
    apiProps: [
      { name: "icon", type: "IconName", description: "Icon to display in the alert" },
      { name: "title", type: "&str", description: "Alert title text" },
      { name: "description", type: "&str", description: "Alert description text" },
      { name: "variant", type: "AlertVariant", description: "Visual variant of the alert" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["alert_style(tokens, variant)"],
    builderFunctions: ["styled(icon, title, description, variant, tokens)", "info(title, description, tokens)", "error(title, description, tokens)", "success(title, description, tokens)", "warning(title, description, tokens)"],
  },

  avatar: {
    name: "Avatar",
    slug: "avatar",
    description: "Circular user avatar with initials fallback when no image is available.",
    category: "Data Display",
    preview: (
      <div className="flex items-center gap-4">
        <div className="w-8 h-8 rounded-full bg-muted border border-border flex items-center justify-center text-xs text-muted-foreground font-medium">JD</div>
        <div className="w-10 h-10 rounded-full bg-muted border border-border flex items-center justify-center text-sm text-muted-foreground font-medium">AB</div>
        <div className="w-14 h-14 rounded-full bg-muted border border-border flex items-center justify-center text-lg text-muted-foreground font-medium">TN</div>
        <div className="w-10 h-10 rounded-full bg-primary flex items-center justify-center text-sm text-primary-foreground font-medium">FL</div>
      </div>
    ),
    variants: [
      { label: "Small", element: <div className="w-8 h-8 rounded-full bg-muted border border-border flex items-center justify-center text-xs text-muted-foreground font-medium">SM</div> },
      { label: "Default", element: <div className="w-10 h-10 rounded-full bg-muted border border-border flex items-center justify-center text-sm text-muted-foreground font-medium">DF</div> },
      { label: "Large", element: <div className="w-14 h-14 rounded-full bg-muted border border-border flex items-center justify-center text-lg text-muted-foreground font-medium">LG</div> },
      { label: "Primary", element: <div className="w-10 h-10 rounded-full bg-primary flex items-center justify-center text-sm text-primary-foreground font-medium">PR</div> },
    ],
    code: `use floe_ui::components::avatar::{self, AvatarSize};

// Default avatar with initials
let av = avatar::initials("John Doe", AvatarSize::Default, &tokens);

// Small avatar
let av_sm = avatar::initials("Jane", AvatarSize::Sm, &tokens);

// Large avatar
let av_lg = avatar::initials("Thang Nguyen", AvatarSize::Lg, &tokens);

// Primary-colored avatar
let av_primary = avatar::initials_primary("Floe", AvatarSize::Default, &tokens);`,
    apiProps: [
      { name: "name", type: "&str", description: "Full name — initials are auto-extracted" },
      { name: "size", type: "AvatarSize", default: "Default", description: "Size preset: Sm (32px), Default (40px), Lg (56px)" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)", "primary_style(tokens)"],
    builderFunctions: ["initials(name, size, tokens)", "initials_primary(name, size, tokens)"],
  },

  badge: {
    name: "Badge",
    slug: "badge",
    description: "Small status labels and tags with multiple visual variants.",
    category: "Data Display",
    preview: (
      <div className="flex flex-wrap gap-3">
        <span className="inline-flex items-center rounded-full bg-primary text-primary-foreground px-2.5 py-0.5 text-xs font-medium">Default</span>
        <span className="inline-flex items-center rounded-full bg-secondary text-secondary-foreground px-2.5 py-0.5 text-xs font-medium">Secondary</span>
        <span className="inline-flex items-center rounded-full border border-border text-foreground px-2.5 py-0.5 text-xs font-medium">Outline</span>
        <span className="inline-flex items-center rounded-full bg-destructive text-destructive-foreground px-2.5 py-0.5 text-xs font-medium">Destructive</span>
      </div>
    ),
    variants: [
      { label: "Default", element: <span className="inline-flex items-center rounded-full bg-primary text-primary-foreground px-2.5 py-0.5 text-xs font-medium">Default</span> },
      { label: "Secondary", element: <span className="inline-flex items-center rounded-full bg-secondary text-secondary-foreground px-2.5 py-0.5 text-xs font-medium">Secondary</span> },
      { label: "Outline", element: <span className="inline-flex items-center rounded-full border border-border text-foreground px-2.5 py-0.5 text-xs font-medium">Outline</span> },
      { label: "Destructive", element: <span className="inline-flex items-center rounded-full bg-destructive text-destructive-foreground px-2.5 py-0.5 text-xs font-medium">Destructive</span> },
    ],
    code: `use floe_ui::components::badge;

// Default (primary) badge
let b1 = badge::primary("New", &tokens);

// Secondary badge
let b2 = badge::secondary("Draft", &tokens);

// Outline badge
let b3 = badge::outline("v1.0", &tokens);

// Destructive badge
let b4 = badge::destructive("Removed", &tokens);`,
    apiProps: [
      { name: "label", type: "&str", description: "Badge text content" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)", "secondary_style(tokens)", "outline_style(tokens)", "destructive_style(tokens)"],
    builderFunctions: ["primary(label, tokens)", "secondary(label, tokens)", "outline(label, tokens)", "destructive(label, tokens)"],
  },

  button: {
    name: "Button",
    slug: "button",
    description: "Beautifully styled buttons with multiple variants and sizes.",
    category: "Form",
    preview: (
      <div className="flex flex-wrap gap-3">
        <button className={`bg-primary text-primary-foreground shadow hover:bg-primary/90 ${btnBase}`}>Primary</button>
        <button className={`bg-secondary text-secondary-foreground hover:bg-secondary/80 ${btnBase}`}>Secondary</button>
        <button className={`border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground ${btnBase}`}>Outline</button>
        <button className={`hover:bg-accent hover:text-accent-foreground ${btnBase}`}>Ghost</button>
        <button className={`bg-destructive text-destructive-foreground shadow hover:bg-destructive/90 ${btnBase}`}>Destructive</button>
        <button className={`text-primary underline-offset-4 hover:underline ${btnBase}`}>Link</button>
      </div>
    ),
    variants: [
      { label: "Primary", element: <button className={`bg-primary text-primary-foreground shadow hover:bg-primary/90 ${btnBase}`}>Primary</button> },
      { label: "Secondary", element: <button className={`bg-secondary text-secondary-foreground hover:bg-secondary/80 ${btnBase}`}>Secondary</button> },
      { label: "Outline", element: <button className={`border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground ${btnBase}`}>Outline</button> },
      { label: "Ghost", element: <button className={`hover:bg-accent hover:text-accent-foreground ${btnBase}`}>Ghost</button> },
      { label: "Destructive", element: <button className={`bg-destructive text-destructive-foreground shadow hover:bg-destructive/90 ${btnBase}`}>Destructive</button> },
      { label: "Link", element: <button className={`text-primary underline-offset-4 hover:underline ${btnBase}`}>Link</button> },
    ],
    code: `use floe_ui::components::button::{self, ButtonSize};

// Primary (default) button
let btn = button::primary("Click me", &tokens)
    .on_press(Message::Clicked);

// Secondary button
let btn2 = button::secondary("Cancel", &tokens)
    .on_press(Message::Cancel);

// Outline button
let btn3 = button::outline("Details", &tokens)
    .on_press(Message::Details);

// Ghost button
let btn4 = button::ghost("More", &tokens)
    .on_press(Message::More);

// Destructive button
let btn5 = button::destructive("Delete", &tokens)
    .on_press(Message::Delete);

// Link button
let btn6 = button::link("Learn more", &tokens)
    .on_press(Message::LearnMore);

// Custom size
let btn_lg = button::primary_sized("Large", &tokens, ButtonSize::Lg)
    .on_press(Message::Clicked);`,
    apiProps: [
      { name: "label", type: "&str", description: "Button text content" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
      { name: "size", type: "ButtonSize", default: "Default", description: "Size preset: Sm, Default, Lg, Icon" },
    ],
    styleFunctions: ["primary_style(tokens)", "secondary_style(tokens)", "outline_style(tokens)", "ghost_style(tokens)", "destructive_style(tokens)", "link_style(tokens)"],
    builderFunctions: ["primary(label, tokens)", "secondary(label, tokens)", "outline(label, tokens)", "ghost(label, tokens)", "destructive(label, tokens)", "link(label, tokens)", "primary_sized(label, tokens, size)"],
  },

  card: {
    name: "Card",
    slug: "card",
    description: "A styled container with optional header, content, and footer sections.",
    category: "Layout",
    preview: (
      <div className="flex flex-col md:flex-row gap-4">
        <div className="rounded-xl border border-border bg-card text-card-foreground shadow-sm p-6 w-full max-w-[280px]">
          <h3 className="font-semibold leading-none tracking-tight mb-2">Project Alpha</h3>
          <p className="text-sm text-muted-foreground mb-4">Deploy your new project in one-click.</p>
          <button className={`bg-primary text-primary-foreground shadow hover:bg-primary/90 ${btnBase} w-full`}>Deploy</button>
        </div>
        <div className="rounded-xl border border-border bg-card text-card-foreground shadow-lg p-6 w-full max-w-[280px]">
          <h3 className="font-semibold leading-none tracking-tight mb-2">Elevated Card</h3>
          <p className="text-sm text-muted-foreground">Stronger shadow for floating surfaces.</p>
        </div>
      </div>
    ),
    variants: [
      { label: "Default", element: <div className="rounded-xl border border-border bg-card text-card-foreground shadow-sm p-6 w-full max-w-[260px]"><h3 className="font-semibold mb-1">Default</h3><p className="text-sm text-muted-foreground">Standard card with border and subtle shadow.</p></div> },
      { label: "Elevated", element: <div className="rounded-xl border border-border bg-card text-card-foreground shadow-lg p-6 w-full max-w-[260px]"><h3 className="font-semibold mb-1">Elevated</h3><p className="text-sm text-muted-foreground">Stronger shadow for floating panels.</p></div> },
      { label: "Ghost", element: <div className="rounded-xl bg-card/50 text-card-foreground p-6 w-full max-w-[260px]"><h3 className="font-semibold mb-1">Ghost</h3><p className="text-sm text-muted-foreground">No border, no shadow, tinted surface.</p></div> },
    ],
    code: `use floe_ui::components::card;
use iced::widget::{text, column};

// Simple card
let c = card::styled(text("Hello!"), &tokens);

// Elevated card
let c2 = card::elevated(text("Elevated card"), &tokens);

// Card with header, content, and footer
let c3 = card::sectioned(
    Some(text("Card Title").size(18).into()),
    text("Card content goes here.").into(),
    Some(text("Footer").size(12).into()),
    &tokens,
);`,
    apiProps: [
      { name: "content", type: "impl Into<Element>", description: "Card body content" },
      { name: "header", type: "Option<Element>", description: "Optional header section (sectioned variant)" },
      { name: "footer", type: "Option<Element>", description: "Optional footer section (sectioned variant)" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)", "elevated_style(tokens)", "ghost_style(tokens)"],
    builderFunctions: ["styled(content, tokens)", "elevated(content, tokens)", "sectioned(header, content, footer, tokens)"],
  },

  checkbox: {
    name: "Checkbox",
    slug: "checkbox",
    description: "A styled checkbox with label — wraps Iced's native checkbox widget.",
    category: "Form",
    preview: (
      <div className="flex flex-col gap-3">
        <label className="flex items-center gap-2 cursor-pointer"><span className="w-[18px] h-[18px] rounded-sm border-[1.5px] border-input bg-transparent flex items-center justify-center" /><span className="text-sm">Unchecked</span></label>
        <label className="flex items-center gap-2 cursor-pointer"><span className="w-[18px] h-[18px] rounded-sm border-[1.5px] border-primary bg-primary flex items-center justify-center"><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" className="text-primary-foreground"><path d="M20 6 9 17l-5-5"/></svg></span><span className="text-sm">Checked</span></label>
        <label className="flex items-center gap-2 cursor-pointer opacity-50"><span className="w-[18px] h-[18px] rounded-sm border-[1.5px] border-input/50 bg-transparent flex items-center justify-center" /><span className="text-sm">Disabled</span></label>
      </div>
    ),
    code: `use floe_ui::components::checkbox;

// Create a styled checkbox
let cb = checkbox::styled("Accept terms", is_checked, &tokens)
    .on_toggle(Message::ToggleTerms);`,
    apiProps: [
      { name: "label", type: "&str", description: "Label text for the checkbox" },
      { name: "is_checked", type: "bool", description: "Current checked state" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(label, is_checked, tokens)"],
  },

  dropdown: {
    name: "Dropdown Menu",
    slug: "dropdown",
    description: "A floating menu for actions — custom widget with overlay support.",
    category: "Navigation",
    preview: (
      <div className="relative inline-block text-left w-full max-w-[220px]">
        <div className="w-full rounded-md border border-border bg-popover text-popover-foreground shadow-md p-1">
          <div className="px-2 py-1.5 text-xs font-medium text-muted-foreground">My Account</div>
          <button className="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground">Profile<span className="ml-auto text-[10px] tracking-widest text-muted-foreground">⇧⌘P</span></button>
          <button className="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground">Billing<span className="ml-auto text-[10px] tracking-widest text-muted-foreground">⌘B</span></button>
          <div className="my-1 h-px bg-muted" />
          <button className="relative flex w-full cursor-default select-none items-center rounded-sm px-2 py-1.5 text-sm outline-none transition-colors hover:bg-accent hover:text-accent-foreground">Log out<span className="ml-auto text-[10px] tracking-widest text-muted-foreground">⇧⌘Q</span></button>
        </div>
      </div>
    ),
    code: `use floe_ui::components::{dropdown, button, separator};
use iced::widget::{column, text};

// Create the dropdown menu content
let menu = dropdown::dropdown_menu(
    column![
        dropdown::dropdown_item(text("Profile"), Message::Profile, &tokens),
        dropdown::dropdown_item(text("Billing"), Message::Billing, &tokens),
        separator::horizontal(&tokens),
        dropdown::dropdown_item(text("Log out"), Message::Logout, &tokens),
    ].spacing(2),
    &tokens,
).width(220.0);

// Create the dropdown widget with trigger
let dropdown_widget = dropdown::Dropdown::new(
    button::outline("Open Menu", &tokens)
        .on_press(Message::ToggleMenu),
    menu,
    self.is_menu_open,
).on_dismiss(Message::CloseMenu);`,
    apiProps: [
      { name: "base", type: "impl Into<Element>", description: "Trigger element (usually a button)" },
      { name: "menu", type: "impl Into<Element>", description: "Menu content to display in overlay" },
      { name: "is_open", type: "bool", description: "Whether the menu is currently visible" },
      { name: "on_dismiss", type: "Message", description: "Message sent when clicking outside the menu" },
    ],
    styleFunctions: ["menu_style(tokens)", "item_style(tokens)"],
    builderFunctions: ["Dropdown::new(base, menu, is_open)", "dropdown_menu(content, tokens)", "dropdown_item(content, on_press, tokens)"],
  },

  input: {
    name: "Input",
    slug: "input",
    description: "A styled text input field with focus ring and placeholder styling.",
    category: "Form",
    preview: (
      <div className="w-full max-w-sm space-y-4">
        <div className="space-y-2">
          <label className="text-sm font-medium">Email</label>
          <input type="email" placeholder="m@example.com" className={inputBase} />
          <p className="text-[0.8rem] text-muted-foreground">Enter your email address.</p>
        </div>
      </div>
    ),
    variants: [
      { label: "Default", element: <input type="text" placeholder="Default input..." className={`${inputBase} max-w-[240px]`} /> },
      { label: "Ghost", element: <input type="text" placeholder="Ghost input..." className="flex h-9 w-full max-w-[240px] rounded-md bg-transparent px-3 py-1 text-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring hover:bg-accent/50" /> },
    ],
    code: `use floe_ui::components::input;

// Default styled input
let field = input::styled("Email…", &self.email, &tokens)
    .on_input(Message::EmailChanged);

// Ghost input (minimal styling)
let ghost = input::ghost("Search…", &self.search, &tokens)
    .on_input(Message::SearchChanged);`,
    apiProps: [
      { name: "placeholder", type: "&str", description: "Placeholder text" },
      { name: "value", type: "&str", description: "Current text value" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)", "ghost_style(tokens)"],
    builderFunctions: ["styled(placeholder, value, tokens)", "ghost(placeholder, value, tokens)"],
  },

  pagination: {
    name: "Pagination",
    slug: "pagination",
    description: "Navigation controls for paginated data with previous/next and page numbers.",
    category: "Navigation",
    preview: (
      <div className="flex items-center gap-1">
        <button className={`hover:bg-accent hover:text-accent-foreground h-9 px-3 py-2 rounded-md text-sm font-medium transition-colors flex items-center gap-1 text-muted-foreground`}>
          <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m15 18-6-6 6-6"/></svg>Previous
        </button>
        <button className={`border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground h-9 w-9 rounded-md text-sm font-medium transition-colors`}>1</button>
        <button className={`hover:bg-accent hover:text-accent-foreground h-9 w-9 rounded-md text-sm font-medium transition-colors`}>2</button>
        <button className={`hover:bg-accent hover:text-accent-foreground h-9 w-9 rounded-md text-sm font-medium transition-colors`}>3</button>
        <span className="px-3 text-sm text-muted-foreground">...</span>
        <button className={`hover:bg-accent hover:text-accent-foreground h-9 px-3 py-2 rounded-md text-sm font-medium transition-colors flex items-center gap-1 text-muted-foreground`}>
          Next<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="m9 18 6-6-6-6"/></svg>
        </button>
      </div>
    ),
    code: `use floe_ui::components::pagination;

let nav = pagination::pagination([
    pagination::previous(Some(Message::PrevPage), &tokens),
    pagination::item("1", true, Message::GoToPage(1), &tokens),
    pagination::item("2", false, Message::GoToPage(2), &tokens),
    pagination::item("3", false, Message::GoToPage(3), &tokens),
    pagination::ellipsis(&tokens),
    pagination::next(Some(Message::NextPage), &tokens),
]);`,
    apiProps: [
      { name: "label", type: "&str", description: "Page number label" },
      { name: "is_active", type: "bool", description: "Whether this is the current page" },
      { name: "on_press", type: "Message", description: "Message sent on click" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: [],
    builderFunctions: ["pagination(items)", "item(label, is_active, on_press, tokens)", "previous(on_press, tokens)", "next(on_press, tokens)", "ellipsis(tokens)"],
  },

  progress: {
    name: "Progress",
    slug: "progress",
    description: "A styled progress bar with default, destructive, and success variants.",
    category: "Feedback",
    preview: (
      <div className="flex flex-col gap-4 w-full max-w-md">
        <div><p className="text-sm text-muted-foreground mb-2">Default — 60%</p><div className="h-2 w-full bg-secondary rounded-full overflow-hidden"><div className="h-full bg-primary rounded-full transition-all" style={{ width: "60%" }} /></div></div>
        <div><p className="text-sm text-muted-foreground mb-2">Success — 100%</p><div className="h-2 w-full bg-secondary rounded-full overflow-hidden"><div className="h-full bg-emerald-500 rounded-full transition-all" style={{ width: "100%" }} /></div></div>
        <div><p className="text-sm text-muted-foreground mb-2">Destructive — 30%</p><div className="h-2 w-full bg-secondary rounded-full overflow-hidden"><div className="h-full bg-destructive rounded-full transition-all" style={{ width: "30%" }} /></div></div>
      </div>
    ),
    variants: [
      { label: "Default", element: <div className="h-2 w-full max-w-[200px] bg-secondary rounded-full overflow-hidden"><div className="h-full bg-primary rounded-full" style={{ width: "60%" }} /></div> },
      { label: "Success", element: <div className="h-2 w-full max-w-[200px] bg-secondary rounded-full overflow-hidden"><div className="h-full bg-emerald-500 rounded-full" style={{ width: "80%" }} /></div> },
      { label: "Destructive", element: <div className="h-2 w-full max-w-[200px] bg-secondary rounded-full overflow-hidden"><div className="h-full bg-destructive rounded-full" style={{ width: "40%" }} /></div> },
    ],
    code: `use floe_ui::components::progress;

// Default progress bar
let bar = progress::styled(0.0..=100.0, 60.0, &tokens);

// Destructive-styled bar
let bar_err = progress::destructive(0.0..=100.0, 30.0, &tokens);

// Success-styled bar
let bar_ok = progress::success(0.0..=100.0, 100.0, &tokens);`,
    apiProps: [
      { name: "range", type: "RangeInclusive<f32>", description: "Min..=max range for the progress" },
      { name: "value", type: "f32", description: "Current progress value" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)", "destructive_style(tokens)", "success_style(tokens)"],
    builderFunctions: ["styled(range, value, tokens)", "destructive(range, value, tokens)", "success(range, value, tokens)"],
  },

  radio: {
    name: "Radio",
    slug: "radio",
    description: "A styled radio button — circular with a primary dot when selected.",
    category: "Form",
    preview: (
      <div className="flex flex-col gap-3">
        <label className="flex items-center gap-2 cursor-pointer"><span className="w-[18px] h-[18px] rounded-full border-[1.5px] border-primary bg-primary flex items-center justify-center"><span className="w-2 h-2 rounded-full bg-primary-foreground" /></span><span className="text-sm">Option A</span></label>
        <label className="flex items-center gap-2 cursor-pointer"><span className="w-[18px] h-[18px] rounded-full border-[1.5px] border-input bg-transparent" /><span className="text-sm">Option B</span></label>
        <label className="flex items-center gap-2 cursor-pointer"><span className="w-[18px] h-[18px] rounded-full border-[1.5px] border-input bg-transparent" /><span className="text-sm">Option C</span></label>
      </div>
    ),
    code: `use floe_ui::components::radio;

// Create radio buttons with shared selection state
let r1 = radio::styled_with("Option A", 0, Some(selected), Message::Selected(0), &tokens);
let r2 = radio::styled_with("Option B", 1, Some(selected), Message::Selected(1), &tokens);
let r3 = radio::styled_with("Option C", 2, Some(selected), Message::Selected(2), &tokens);`,
    apiProps: [
      { name: "label", type: "&str", description: "Radio label text" },
      { name: "value", type: "V: Copy + Eq", description: "Value this radio represents" },
      { name: "selected", type: "Option<V>", description: "Currently selected value" },
      { name: "on_click", type: "Message", description: "Message sent when clicked" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(label, value, selected, tokens)", "styled_with(label, value, selected, on_click, tokens)"],
  },

  select: {
    name: "Select",
    slug: "select",
    description: "A styled select dropdown — wraps Iced's pick_list widget.",
    category: "Form",
    preview: (
      <div className="w-full max-w-[220px] relative">
        <select defaultValue="" className="flex h-9 w-full appearance-none rounded-md border border-input bg-background px-3 py-1.5 text-sm shadow-sm transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring cursor-pointer">
          <option value="" disabled>Select a fruit…</option>
          <option value="apple">Apple</option>
          <option value="banana">Banana</option>
          <option value="cherry">Cherry</option>
          <option value="mango">Mango</option>
        </select>
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="absolute right-3 top-2.5 text-muted-foreground pointer-events-none"><path d="m6 9 6 6 6-6"/></svg>
      </div>
    ),
    code: `use floe_ui::components::select;

let options = vec!["Apple", "Banana", "Cherry", "Mango"];
let sel = select::styled(
    options,
    self.selected_fruit.as_ref(),
    Message::FruitSelected,
    &tokens,
);`,
    apiProps: [
      { name: "options", type: "L: Borrow<[T]>", description: "List of options to choose from" },
      { name: "selected", type: "Option<V>", description: "Currently selected option" },
      { name: "on_selected", type: "Fn(T) -> Message", description: "Callback when an option is selected" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(options, selected, on_selected, tokens)"],
  },

  separator: {
    name: "Separator",
    slug: "separator",
    description: "A visual divider — horizontal or vertical rule styled with design tokens.",
    category: "Layout",
    preview: (
      <div className="w-full max-w-md space-y-4">
        <div className="space-y-2">
          <p className="text-sm">Content above</p>
          <div className="h-px w-full bg-border" />
          <p className="text-sm">Content below</p>
        </div>
        <div className="flex items-center gap-4 h-12">
          <span className="text-sm">Left</span>
          <div className="w-px h-full bg-border" />
          <span className="text-sm">Right</span>
        </div>
      </div>
    ),
    variants: [
      { label: "Horizontal", element: <div className="w-full max-w-[200px] space-y-2"><span className="text-xs text-muted-foreground block">Section A</span><div className="h-px w-full bg-border" /><span className="text-xs text-muted-foreground block">Section B</span></div> },
      { label: "Vertical", element: <div className="flex items-center gap-4 h-10"><span className="text-xs text-muted-foreground">Left</span><div className="w-px h-full bg-border" /><span className="text-xs text-muted-foreground">Right</span></div> },
    ],
    code: `use floe_ui::components::separator;

// Horizontal divider
let hr = separator::horizontal(&tokens);

// Vertical divider
let vr = separator::vertical(&tokens);`,
    apiProps: [
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["horizontal(tokens)", "vertical(tokens)"],
  },

  skeleton: {
    name: "Skeleton",
    slug: "skeleton",
    description: "A placeholder for loading state — rectangular or circular shapes.",
    category: "Feedback",
    preview: (
      <div className="flex items-center gap-4">
        <div className="w-12 h-12 rounded-full bg-muted animate-pulse" />
        <div className="space-y-2 flex-1">
          <div className="h-4 w-3/4 bg-muted rounded-md animate-pulse" />
          <div className="h-4 w-1/2 bg-muted rounded-md animate-pulse" />
        </div>
      </div>
    ),
    variants: [
      { label: "Rectangle", element: <div className="h-4 w-[200px] bg-muted rounded-md animate-pulse" /> },
      { label: "Circle", element: <div className="w-12 h-12 rounded-full bg-muted animate-pulse" /> },
    ],
    code: `use floe_ui::components::skeleton;

// Rectangular placeholder
let rect = skeleton::rect(200, 16, &tokens);

// Circular placeholder (e.g. avatar loading)
let circle = skeleton::circle(48, &tokens);`,
    apiProps: [
      { name: "width", type: "impl Into<Length>", description: "Width of the skeleton" },
      { name: "height", type: "impl Into<Length>", description: "Height of the skeleton (rect only)" },
      { name: "diameter", type: "impl Into<Length>", description: "Diameter of the circle (circle only)" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)", "circle_style(tokens)"],
    builderFunctions: ["rect(width, height, tokens)", "circle(diameter, tokens)"],
  },

  slider: {
    name: "Slider",
    slug: "slider",
    description: "A styled range slider with primary-colored track and circular handle.",
    category: "Form",
    preview: (
      <div className="w-full max-w-md space-y-2">
        <div className="relative w-full h-5 flex items-center">
          <div className="absolute h-1 w-full bg-secondary rounded-full" />
          <div className="absolute h-1 bg-primary rounded-full" style={{ width: "60%" }} />
          <div className="absolute w-4 h-4 rounded-full bg-primary border-2 border-primary shadow-sm" style={{ left: "calc(60% - 8px)" }} />
        </div>
        <p className="text-sm text-muted-foreground">Value: 60%</p>
      </div>
    ),
    code: `use floe_ui::components::slider;

let s = slider::styled(0.0..=100.0, self.value, Message::SliderChanged, &tokens);`,
    apiProps: [
      { name: "range", type: "RangeInclusive<f32>", description: "Min..=max range" },
      { name: "value", type: "f32", description: "Current slider value" },
      { name: "on_change", type: "Fn(f32) -> Message", description: "Callback on value change" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(range, value, on_change, tokens)"],
  },

  table: {
    name: "Table",
    slug: "table",
    description: "A structured data table with headers and rows, built from Iced primitives.",
    category: "Data Display",
    preview: (
      <div className="w-full max-w-lg border border-border rounded-md overflow-hidden text-sm">
        <div className="flex bg-muted/30 border-b border-border px-4 py-3 font-medium text-muted-foreground">
          <div className="flex-1">Invoice</div><div className="flex-1">Status</div><div className="flex-1 text-right">Amount</div>
        </div>
        <div className="flex px-4 py-3 border-b border-border/50"><div className="flex-1">INV-001</div><div className="flex-1"><span className="inline-flex items-center rounded-full bg-emerald-500/10 text-emerald-400 px-2 py-0.5 text-xs font-medium">Paid</span></div><div className="flex-1 text-right">$250.00</div></div>
        <div className="flex px-4 py-3 border-b border-border/50"><div className="flex-1">INV-002</div><div className="flex-1"><span className="inline-flex items-center rounded-full bg-yellow-500/10 text-yellow-400 px-2 py-0.5 text-xs font-medium">Pending</span></div><div className="flex-1 text-right">$150.00</div></div>
        <div className="flex px-4 py-3"><div className="flex-1">INV-003</div><div className="flex-1"><span className="inline-flex items-center rounded-full bg-red-500/10 text-red-400 px-2 py-0.5 text-xs font-medium">Overdue</span></div><div className="flex-1 text-right">$350.00</div></div>
      </div>
    ),
    code: `use floe_ui::components::table;
use iced::widget::row;

let tbl = table::table(
    // Headers
    [
        table::header_cell("Invoice", &tokens),
        table::header_cell("Status", &tokens),
        table::header_cell("Amount", &tokens),
    ],
    // Rows
    [
        row![
            table::cell("INV-001", &tokens),
            table::cell("Paid", &tokens),
            table::cell("$250.00", &tokens),
        ],
        row![
            table::cell("INV-002", &tokens),
            table::cell("Pending", &tokens),
            table::cell("$150.00", &tokens),
        ],
    ],
    &tokens,
);`,
    apiProps: [
      { name: "headers", type: "impl IntoIterator<Item = Element>", description: "Header cell elements" },
      { name: "rows", type: "impl IntoIterator<Item = Row>", description: "Data rows" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["table_container_style(tokens)", "header_style(tokens)", "row_style(tokens)"],
    builderFunctions: ["table(headers, rows, tokens)", "cell(content, tokens)", "header_cell(content, tokens)"],
  },

  tabs: {
    name: "Tabs",
    slug: "tabs",
    description: "Navigation tabs for switching views — container with active/inactive states.",
    category: "Navigation",
    preview: (
      <div className="w-full max-w-md">
        <div className="inline-flex h-9 items-center justify-center rounded-lg bg-muted p-1 text-muted-foreground">
          <button className="inline-flex items-center justify-center whitespace-nowrap rounded-md px-4 py-1 text-sm font-medium bg-background text-foreground shadow-sm transition-all">Account</button>
          <button className="inline-flex items-center justify-center whitespace-nowrap rounded-md px-4 py-1 text-sm font-medium hover:bg-background/50 hover:text-foreground transition-all">Password</button>
          <button className="inline-flex items-center justify-center whitespace-nowrap rounded-md px-4 py-1 text-sm font-medium hover:bg-background/50 hover:text-foreground transition-all">Settings</button>
        </div>
      </div>
    ),
    code: `use floe_ui::components::tabs;

// Create a tab list with individual tabs
let tab_list = tabs::list(
    [
        tabs::tab("Account", self.active_tab == 0, Message::SetTab(0), &tokens),
        tabs::tab("Password", self.active_tab == 1, Message::SetTab(1), &tokens),
        tabs::tab("Settings", self.active_tab == 2, Message::SetTab(2), &tokens),
    ],
    &tokens,
);`,
    apiProps: [
      { name: "label", type: "&str", description: "Tab label text" },
      { name: "is_active", type: "bool", description: "Whether this tab is currently active" },
      { name: "on_press", type: "Message", description: "Message sent when tab is clicked" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["tab_list_style(tokens)", "tab_active_style(tokens)", "tab_inactive_style(tokens)"],
    builderFunctions: ["list(tabs, tokens)", "tab(label, is_active, on_press, tokens)"],
  },

  textarea: {
    name: "Textarea",
    slug: "textarea",
    description: "A styled multi-line text editor wrapping Iced's text_editor widget.",
    category: "Form",
    preview: (
      <div className="w-full max-w-md">
        <textarea
          placeholder="Type your message here…"
          className="flex min-h-[120px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm transition-colors placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring resize-none"
          readOnly
        />
      </div>
    ),
    code: `use floe_ui::components::textarea;
use iced::widget::text_editor;

// Create editor content in your state
let content = text_editor::Content::new();

// Create styled textarea
let editor = textarea::styled(&self.content, Message::EditorAction, &tokens);`,
    apiProps: [
      { name: "content", type: "&text_editor::Content", description: "Editor content state" },
      { name: "on_action", type: "Fn(Action) -> Message", description: "Callback on editor actions" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(content, on_action, tokens)"],
  },

  toggle: {
    name: "Toggle",
    slug: "toggle",
    description: "A switch/toggle control — styled wrapper around Iced's toggler widget.",
    category: "Form",
    preview: (
      <div className="flex flex-col gap-4">
        <div className="flex items-center gap-3">
          <div className="w-[42px] h-[22px] rounded-full bg-primary relative cursor-pointer"><div className="absolute w-[18px] h-[18px] rounded-full bg-primary-foreground top-[2px] right-[2px] transition-all" /></div>
          <span className="text-sm">Enabled</span>
        </div>
        <div className="flex items-center gap-3">
          <div className="w-[42px] h-[22px] rounded-full bg-input relative cursor-pointer"><div className="absolute w-[18px] h-[18px] rounded-full bg-foreground top-[2px] left-[2px] transition-all" /></div>
          <span className="text-sm">Disabled</span>
        </div>
      </div>
    ),
    code: `use floe_ui::components::toggle;

// Toggle with label
let t = toggle::styled(Some("Airplane mode".to_string()), self.is_toggled, &tokens)
    .on_toggle(Message::Toggled);

// Toggle without label
let t2 = toggle::styled(None::<String>, self.dark_mode, &tokens)
    .on_toggle(Message::DarkModeToggled);`,
    apiProps: [
      { name: "label", type: "impl Into<Option<String>>", description: "Optional label text" },
      { name: "is_toggled", type: "bool", description: "Current toggle state" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(label, is_toggled, tokens)"],
  },

  "toggle-group": {
    name: "Toggle Group",
    slug: "toggle-group",
    description: "A set of interconnected toggle buttons with shared border radius.",
    category: "Form",
    preview: (
      <div className="inline-flex rounded-md border border-border overflow-hidden">
        <button className="px-4 py-2 text-sm font-medium bg-accent text-accent-foreground">Bold</button>
        <button className="px-4 py-2 text-sm font-medium hover:bg-accent/50 transition-colors border-l border-border">Italic</button>
        <button className="px-4 py-2 text-sm font-medium hover:bg-accent/50 transition-colors border-l border-border">Underline</button>
      </div>
    ),
    code: `use floe_ui::components::toggle_group::{self, ItemPosition};
use iced::widget::text;

let group = toggle_group::group([
    toggle_group::item(
        text("Bold").size(14),
        self.active == 0,
        ItemPosition::First,
        Message::SetFormat(0),
        &tokens,
    ),
    toggle_group::item(
        text("Italic").size(14),
        self.active == 1,
        ItemPosition::Middle,
        Message::SetFormat(1),
        &tokens,
    ),
    toggle_group::item(
        text("Underline").size(14),
        self.active == 2,
        ItemPosition::Last,
        Message::SetFormat(2),
        &tokens,
    ),
]);`,
    apiProps: [
      { name: "content", type: "impl Into<Element>", description: "Item content (usually text)" },
      { name: "is_active", type: "bool", description: "Whether this item is selected" },
      { name: "position", type: "ItemPosition", description: "Position: First, Middle, Last, Single" },
      { name: "on_press", type: "Message", description: "Message sent when clicked" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["item_active_style(position, tokens)", "item_inactive_style(position, tokens)"],
    builderFunctions: ["group(items)", "item(content, is_active, position, on_press, tokens)"],
  },

  tooltip: {
    name: "Tooltip",
    slug: "tooltip",
    description: "A popover-styled tooltip that appears on hover with configurable position.",
    category: "Feedback",
    preview: (
      <div className="flex items-center gap-8">
        <div className="relative group">
          <button className={`border border-input bg-background shadow-sm hover:bg-accent hover:text-accent-foreground ${btnBase}`}>Hover me</button>
          <div className="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-1.5 rounded-md border border-border bg-popover text-popover-foreground shadow-md text-xs whitespace-nowrap opacity-0 group-hover:opacity-100 transition-opacity">
            This is a tooltip
          </div>
        </div>
      </div>
    ),
    code: `use floe_ui::components::tooltip;
use iced::widget::tooltip::Position;

let widget = tooltip::styled(
    button::primary("Hover me", &tokens),
    "This is a tooltip",
    Position::Top,
    &tokens,
);`,
    apiProps: [
      { name: "content", type: "impl Into<Element>", description: "Element to attach tooltip to" },
      { name: "tip", type: "impl ToString", description: "Tooltip text content" },
      { name: "position", type: "Position", description: "Position: Top, Bottom, Left, Right, FollowCursor" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: ["default_style(tokens)"],
    builderFunctions: ["styled(content, tip, position, tokens)"],
  },

  typography: {
    name: "Typography",
    slug: "typography",
    description: "Standardized text styles — headings, paragraphs, lead, and muted text.",
    category: "Data Display",
    preview: (
      <div className="space-y-4 w-full max-w-md">
        <h1 className="text-4xl font-bold tracking-tight">Heading 1</h1>
        <h2 className="text-3xl font-semibold tracking-tight">Heading 2</h2>
        <h3 className="text-2xl font-semibold tracking-tight">Heading 3</h3>
        <h4 className="text-xl font-semibold tracking-tight">Heading 4</h4>
        <p className="text-base leading-7">This is a standard paragraph with comfortable line height.</p>
        <p className="text-xl text-muted-foreground">This is lead text for introductions.</p>
        <p className="text-sm text-muted-foreground">This is muted small text.</p>
      </div>
    ),
    code: `use floe_ui::components::typography;

let h1 = typography::h1("Page Title", &tokens);
let h2 = typography::h2("Section", &tokens);
let h3 = typography::h3("Subsection", &tokens);
let h4 = typography::h4("Detail", &tokens);
let body = typography::p("Paragraph text.", &tokens);
let intro = typography::lead("Lead text for intros.", &tokens);
let big = typography::large("Large text.", &tokens);
let sm = typography::small("Small text.", &tokens);
let dim = typography::muted("Muted caption.", &tokens);`,
    apiProps: [
      { name: "content", type: "&str", description: "Text content" },
      { name: "tokens", type: "&DesignTokens", description: "Design tokens for theming" },
    ],
    styleFunctions: [],
    builderFunctions: ["h1(content, tokens)", "h2(content, tokens)", "h3(content, tokens)", "h4(content, tokens)", "p(content, tokens)", "lead(content, tokens)", "large(content, tokens)", "small(content, tokens)", "muted(content, tokens)"],
  },
};

// ── Helper: Get all components as array ────────────────────────────────

export function getAllComponents(): ComponentData[] {
  return Object.values(componentsData);
}

export function getComponentsByCategory(category: ComponentCategory): ComponentData[] {
  return getAllComponents().filter(c => c.category === category);
}

export const categories: ComponentCategory[] = ["Form", "Layout", "Data Display", "Feedback", "Navigation"];
