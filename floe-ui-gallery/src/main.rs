//! Floe UI Gallery — interactive showcase of all components.

use floe_ui::components::{
    alert, avatar, badge, button, card, checkbox, icon, input, pagination, progress, radio, select,
    separator, skeleton, slider, table, tabs, textarea, toggle, toggle_group, tooltip, typography,
};
use floe_ui::prelude::*;

use iced::widget::text_editor;
use iced::widget::tooltip::Position;
use iced::widget::{column, container, pick_list, row, text, Column, Space};
use iced::{
    Element,
    Length::{Fill, FillPortion, Fixed},
    Theme,
};

fn main() -> iced::Result {
    iced::application(Gallery::default, Gallery::update, Gallery::view)
        .title("Floe UI Gallery")
        .theme(Gallery::theme)
        .font(floe_ui::components::icon::LUCIDE_FONT_BYTES)
        .window_size((1100.0, 750.0))
        .run()
}

// ── State ───────────────────────────────────────────────────────────────

fn view_usage<'a>(code_str: &'a str, tokens: &'a DesignTokens) -> Element<'a, Message> {
    iced::widget::container(
        iced::widget::text(code_str)
            .font(iced::Font::MONOSPACE)
            .size(13)
            .color(tokens.foreground),
    )
    .width(iced::Length::Fill)
    .padding(16)
    .style(move |_theme: &Theme| iced::widget::container::Style {
        background: Some(iced::Background::Color(tokens.muted)),
        border: iced::Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_md.into(),
        },
        ..Default::default()
    })
    .into()
}

struct Gallery {
    floe_theme: FloeTheme,
    selected_palette: PaletteChoice,
    input_value: String,
    toggle_value: bool,
    active_section: Section,
    checkbox_a: bool,
    checkbox_b: bool,
    checkbox_c: bool,
    radio_value: Option<RadioChoice>,
    slider_value: f32,
    progress_value: f32,
    select_value: Option<String>,
    textarea_content: text_editor::Content,
    active_tab: usize,
    toggle_group_value: String,
    table_rows_selected: [bool; 4],
    dropdown_open: Option<usize>,
    showcase_dropdown_open: bool,
}

impl Default for Gallery {
    fn default() -> Self {
        Self {
            floe_theme: FloeTheme::zinc_dark(),
            selected_palette: PaletteChoice::ZincDark,
            input_value: String::new(),
            toggle_value: false,
            active_section: Section::Buttons,
            checkbox_a: true,
            checkbox_b: false,
            checkbox_c: false,
            radio_value: Some(RadioChoice::Option1),
            slider_value: 0.5,
            progress_value: 66.0,
            select_value: None,
            textarea_content: text_editor::Content::with_text("Type your message here..."),
            active_tab: 0,
            toggle_group_value: "center".to_string(),
            table_rows_selected: [false; 4],
            dropdown_open: None,
            showcase_dropdown_open: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RadioChoice {
    Option1,
    Option2,
    Option3,
}

impl std::fmt::Display for RadioChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Option1 => write!(f, "Option 1"),
            Self::Option2 => write!(f, "Option 2"),
            Self::Option3 => write!(f, "Option 3"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Section {
    Icons,
    Typography,
    Buttons,
    Inputs,
    Textareas,
    Selects,
    Checkboxes,
    Radios,
    Toggles,
    ToggleGroups,
    Sliders,
    Tabs,
    Cards,
    Tables,
    Badges,
    Avatars,
    Alerts,
    Progress,
    Skeletons,
    Pagination,
    Dropdowns,
}

impl Section {
    const ALL: &'static [Self] = &[
        Self::Icons,
        Self::Typography,
        Self::Buttons,
        Self::Inputs,
        Self::Textareas,
        Self::Selects,
        Self::Checkboxes,
        Self::Radios,
        Self::Toggles,
        Self::ToggleGroups,
        Self::Sliders,
        Self::Tabs,
        Self::Cards,
        Self::Tables,
        Self::Badges,
        Self::Avatars,
        Self::Alerts,
        Self::Progress,
        Self::Skeletons,
        Self::Pagination,
        Self::Dropdowns,
    ];

    fn label(&self) -> &'static str {
        match self {
            Self::Icons => "Icons",
            Self::Typography => "Typography",
            Self::Buttons => "Buttons",
            Self::Inputs => "Inputs",
            Self::Textareas => "Textarea",
            Self::Selects => "Select",
            Self::Checkboxes => "Checkbox",
            Self::Radios => "Radio",
            Self::Toggles => "Toggle",
            Self::ToggleGroups => "Toggle Group",
            Self::Sliders => "Slider",
            Self::Tabs => "Tabs",
            Self::Cards => "Cards",
            Self::Tables => "Table",
            Self::Badges => "Badges",
            Self::Avatars => "Avatar",
            Self::Alerts => "Alert",
            Self::Progress => "Progress",
            Self::Skeletons => "Skeleton",
            Self::Pagination => "Pagination",
            Self::Dropdowns => "Dropdown Menu",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PaletteChoice {
    ZincDark,
    ZincLight,
    CustomEmerald,
}

impl PaletteChoice {
    const ALL: &'static [Self] = &[Self::ZincDark, Self::ZincLight, Self::CustomEmerald];

    fn to_theme(self) -> FloeTheme {
        match self {
            Self::ZincDark => FloeTheme::zinc_dark(),
            Self::ZincLight => FloeTheme::zinc_light(),
            Self::CustomEmerald => {
                let mut tokens = floe_ui::theme::palette::zinc_dark();
                // Emerald-500: HSL 158, 64%, 52%
                // Override primary and ring with emerald
                tokens.primary = floe_ui::theme::tokens::hsl(158.0, 0.64, 0.52);
                tokens.primary_foreground = floe_ui::theme::tokens::hsl(0.0, 0.0, 0.98);
                tokens.ring = floe_ui::theme::tokens::hsl(158.0, 0.64, 0.52);
                FloeTheme::custom(tokens)
            }
        }
    }
}

impl std::fmt::Display for PaletteChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZincDark => write!(f, "Zinc Dark"),
            Self::ZincLight => write!(f, "Zinc Light"),
            Self::CustomEmerald => write!(f, "Custom (Emerald)"),
        }
    }
}

// ── Messages ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
enum Message {
    PaletteChanged(PaletteChoice),
    SectionChanged(Section),
    InputChanged(String),
    ToggleChanged(bool),
    CheckboxA(bool),
    CheckboxB(bool),
    CheckboxC(bool),
    RadioChanged(RadioChoice),
    SliderChanged(f32),
    SelectChanged(String),
    TextareaAction(text_editor::Action),
    TabChanged(usize),
    ToggleGroupChanged(String),
    TableRowSelected(usize, bool),
    TableSelectAll(bool),
    ToggleDropdown(Option<usize>),
    ToggleShowcaseDropdown(bool),
    Noop,
}

// ── Update ──────────────────────────────────────────────────────────────

impl Gallery {
    fn update(&mut self, message: Message) {
        match message {
            Message::PaletteChanged(p) => {
                self.selected_palette = p;
                self.floe_theme = p.to_theme();
            }
            Message::SectionChanged(s) => {
                self.active_section = s;
            }
            Message::InputChanged(v) => {
                self.input_value = v;
            }
            Message::ToggleChanged(v) => {
                self.toggle_value = v;
            }
            Message::CheckboxA(v) => self.checkbox_a = v,
            Message::CheckboxB(v) => self.checkbox_b = v,
            Message::CheckboxC(v) => self.checkbox_c = v,
            Message::RadioChanged(v) => self.radio_value = Some(v),
            Message::SliderChanged(v) => {
                self.slider_value = v;
                self.progress_value = v * 100.0;
            }
            Message::SelectChanged(v) => self.select_value = Some(v),
            Message::TextareaAction(action) => self.textarea_content.perform(action),
            Message::TabChanged(i) => self.active_tab = i,
            Message::ToggleGroupChanged(v) => self.toggle_group_value = v,
            Message::TableRowSelected(i, v) => self.table_rows_selected[i] = v,
            Message::TableSelectAll(v) => self.table_rows_selected.fill(v),
            Message::ToggleDropdown(v) => self.dropdown_open = v,
            Message::ToggleShowcaseDropdown(v) => self.showcase_dropdown_open = v,
            Message::Noop => {}
        }
    }

    fn theme(&self) -> Theme {
        self.floe_theme.inner.clone()
    }

    // ── View ────────────────────────────────────────────────────────

    fn view(&self) -> Element<'_, Message> {
        let tokens = &self.floe_theme.tokens;

        // ── Header ──────────────────────────────────────────────────
        let header = container(
            row![
                text("❄ Floe UI").size(22),
                Space::new().width(Fill),
                text("Theme: ").size(14),
                pick_list(
                    PaletteChoice::ALL,
                    Some(self.selected_palette),
                    Message::PaletteChanged,
                )
                .text_size(13.0),
            ]
            .spacing(8)
            .align_y(iced::Center),
        )
        .padding([12, 24])
        .width(Fill)
        .style(move |_theme: &Theme| container::Style {
            background: Some(tokens.card.into()),
            border: iced::Border {
                color: tokens.border,
                width: 0.0,
                radius: 0.0.into(),
            },
            shadow: iced::Shadow {
                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                offset: iced::Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
            text_color: Some(tokens.foreground),
            snap: false,
        });

        // ── Sidebar ─────────────────────────────────────────────────
        let sidebar = {
            let mut col = Column::new().spacing(2).padding([16, 8]).width(180);
            for &section in Section::ALL {
                let is_active = section == self.active_section;
                let btn = button::ghost(section.label(), tokens)
                    .on_press(Message::SectionChanged(section))
                    .width(Fill);
                if is_active {
                    col = col.push(
                        container(btn).style(move |_theme: &Theme| container::Style {
                            background: Some(tokens.accent.into()),
                            border: iced::Border {
                                color: iced::Color::TRANSPARENT,
                                width: 0.0,
                                radius: tokens.radius_md.into(),
                            },
                            ..Default::default()
                        }),
                    );
                } else {
                    col = col.push(btn);
                }
            }
            let scrollable_sidebar = iced::widget::scrollable(col).height(Fill);
            container(scrollable_sidebar)
                .height(Fill)
                .style(move |_theme: &Theme| container::Style {
                    background: Some(tokens.card.into()),
                    border: iced::Border {
                        color: tokens.border,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
        };

        // ── Content area ────────────────────────────────────────────
        let content =
            iced::widget::scrollable(container(self.view_section(tokens)).padding(32).width(Fill))
                .height(Fill);

        // ── Layout ──────────────────────────────────────────────────
        column![header, row![sidebar, content].height(Fill)]
            .height(Fill)
            .into()
    }

    fn view_section<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        match self.active_section {
            Section::Icons => self.view_icons(tokens),
            Section::Typography => self.view_typography(tokens),
            Section::Buttons => self.view_buttons(tokens),
            Section::Inputs => self.view_inputs(tokens),
            Section::Textareas => self.view_textareas(tokens),
            Section::Cards => self.view_cards(tokens),
            Section::Badges => self.view_badges(tokens),
            Section::Toggles => self.view_toggles(tokens),
            Section::ToggleGroups => self.view_toggle_groups(tokens),
            Section::Checkboxes => self.view_checkboxes(tokens),
            Section::Radios => self.view_radios(tokens),
            Section::Progress => self.view_progress(tokens),
            Section::Sliders => self.view_sliders(tokens),
            Section::Alerts => self.view_alerts(tokens),
            Section::Avatars => self.view_avatars(tokens),
            Section::Selects => self.view_selects(tokens),
            Section::Tabs => self.view_tabs(tokens),
            Section::Tables => self.view_tables(tokens),
            Section::Skeletons => self.view_skeletons(tokens),
            Section::Pagination => self.view_pagination(tokens),
            Section::Dropdowns => self.view_dropdowns(tokens),
        }
    }

    // ── Section: Buttons ────────────────────────────────────────────

    fn view_buttons<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Button").size(28),
            text("Displays a button or a component that looks like a button.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Variants").size(18),
            row![
                button::primary("Primary", tokens).on_press(Message::Noop),
                button::secondary("Secondary", tokens).on_press(Message::Noop),
                button::outline("Outline", tokens).on_press(Message::Noop),
                button::ghost("Ghost", tokens).on_press(Message::Noop),
                button::destructive("Destructive", tokens).on_press(Message::Noop),
                button::link("Link", tokens).on_press(Message::Noop),
            ]
            .spacing(8)
            .align_y(iced::Center),
            separator::horizontal(tokens),
            text("Sizes").size(18),
            row![
                button::primary_sized(
                    "Small",
                    tokens,
                    floe_ui::components::button::ButtonSize::Sm,
                )
                .on_press(Message::Noop),
                button::primary("Default", tokens).on_press(Message::Noop),
                button::primary_sized(
                    "Large",
                    tokens,
                    floe_ui::components::button::ButtonSize::Lg,
                )
                .on_press(Message::Noop),
            ]
            .spacing(8)
            .align_y(iced::Center),
            separator::horizontal(tokens),
            text("With Tooltip").size(18),
            tooltip::styled(
                button::primary("Hover me", tokens).on_press(Message::Noop),
                "This is a tooltip!",
                Position::Bottom,
                tokens,
            ),
            separator::horizontal(tokens),
            text("Disabled").size(18),
            row![
                button::primary("Primary", tokens),
                button::secondary("Secondary", tokens),
                button::outline("Outline", tokens),
                button::destructive("Destructive", tokens),
            ]
            .spacing(8)
            .align_y(iced::Center),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::button::{self, ButtonSize};

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
    .on_press(Message::Clicked);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Inputs ─────────────────────────────────────────────

    fn view_inputs<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Input").size(28),
            text("Displays a form input field.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            input::styled("Type something…", &self.input_value, tokens)
                .on_input(Message::InputChanged)
                .width(320),
            separator::horizontal(tokens),
            text("Ghost").size(18),
            input::ghost("Search…", &self.input_value, tokens)
                .on_input(Message::InputChanged)
                .width(320),
            separator::horizontal(tokens),
            text("Disabled").size(18),
            input::styled("Disabled input", "Cannot edit", tokens).width(320),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::input;

// Default styled input
let field = input::styled("Email…", &self.email, &tokens)
    .on_input(Message::EmailChanged);

// Ghost input (minimal styling)
let ghost = input::ghost("Search…", &self.search, &tokens)
    .on_input(Message::SearchChanged);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Cards ──────────────────────────────────────────────

    fn view_cards<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Card").size(28),
            text("Displays a card with header, content, and footer.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default Card").size(18),
            card::styled(
                column![
                    text("Card Title").size(18),
                    text("This is a card with some descriptive content inside.")
                        .size(14)
                        .color(tokens.muted_foreground),
                ]
                .spacing(8),
                tokens,
            )
            .width(400),
            separator::horizontal(tokens),
            text("Elevated Card").size(18),
            card::elevated(
                column![
                    text("Elevated Card").size(18),
                    text("This card has a stronger shadow for a floating effect.")
                        .size(14)
                        .color(tokens.muted_foreground),
                ]
                .spacing(8),
                tokens,
            )
            .width(400),
            separator::horizontal(tokens),
            text("Sectioned Card").size(18),
            card::sectioned(
                Some(text("Header Section").size(16).into()),
                column![text(
                    "Main content goes here. This card demonstrates the sectioned layout."
                )
                .size(14)
                .color(tokens.muted_foreground),]
                .into(),
                Some(
                    row![
                        button::outline("Cancel", tokens).on_press(Message::Noop),
                        button::primary("Save", tokens).on_press(Message::Noop),
                    ]
                    .spacing(8)
                    .into(),
                ),
                tokens,
            )
            .width(400),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::card;
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
);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Badges ─────────────────────────────────────────────

    fn view_badges<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Badge").size(28),
            text("Displays a badge or a component that looks like a badge.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Variants").size(18),
            row![
                badge::primary("Default", tokens),
                badge::secondary("Secondary", tokens),
                badge::outline("Outline", tokens),
                badge::destructive("Destructive", tokens),
            ]
            .spacing(8)
            .align_y(iced::Center),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::badge;

// Default (primary) badge
let b1 = badge::primary("New", &tokens);

// Secondary badge
let b2 = badge::secondary("Draft", &tokens);

// Outline badge
let b3 = badge::outline("v1.0", &tokens);

// Destructive badge
let b4 = badge::destructive("Removed", &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Toggles ────────────────────────────────────────────

    fn view_toggles<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Toggle").size(28),
            text("A control that allows the user to toggle between checked and not checked.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            toggle::styled(Some("Airplane Mode".to_string()), self.toggle_value, tokens,)
                .on_toggle(Message::ToggleChanged),
            separator::horizontal(tokens),
            text("Without Label").size(18),
            toggle::styled(None::<String>, self.toggle_value, tokens)
                .on_toggle(Message::ToggleChanged),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::toggle;

// Toggle with label
let t = toggle::styled(Some("Airplane mode".to_string()), self.is_toggled, &tokens)
    .on_toggle(Message::Toggled);

// Toggle without label
let t2 = toggle::styled(None::<String>, self.dark_mode, &tokens)
    .on_toggle(Message::DarkModeToggled);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Checkboxes ─────────────────────────────────────────

    fn view_checkboxes<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Checkbox").size(28),
            text("A control that allows the user to select one or more items.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            column![
                checkbox::styled("Accept terms and conditions", self.checkbox_a, tokens)
                    .on_toggle(Message::CheckboxA),
                checkbox::styled(
                    "Use different settings for desktop",
                    self.checkbox_b,
                    tokens
                )
                .on_toggle(Message::CheckboxB),
                checkbox::styled("Send me notifications", self.checkbox_c, tokens)
                    .on_toggle(Message::CheckboxC),
            ]
            .spacing(12),
            separator::horizontal(tokens),
            text("Disabled").size(18),
            checkbox::styled("This checkbox is disabled", true, tokens),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::checkbox;

// Create a styled checkbox
let cb = checkbox::styled("Accept terms", is_checked, &tokens)
    .on_toggle(Message::ToggleTerms);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Radios ─────────────────────────────────────────────

    fn view_radios<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Radio").size(28),
            text("A control that allows the user to select one option from a set.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            column![
                radio::styled_with(
                    "Default option",
                    RadioChoice::Option1,
                    self.radio_value,
                    Message::RadioChanged(RadioChoice::Option1),
                    tokens,
                ),
                radio::styled_with(
                    "Comfortable",
                    RadioChoice::Option2,
                    self.radio_value,
                    Message::RadioChanged(RadioChoice::Option2),
                    tokens,
                ),
                radio::styled_with(
                    "Compact",
                    RadioChoice::Option3,
                    self.radio_value,
                    Message::RadioChanged(RadioChoice::Option3),
                    tokens,
                ),
            ]
            .spacing(12),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::radio;

// Create radio buttons with shared selection state
let r1 = radio::styled_with("Option A", 0, Some(selected), Message::Selected(0), &tokens);
let r2 = radio::styled_with("Option B", 1, Some(selected), Message::Selected(1), &tokens);
let r3 = radio::styled_with("Option C", 2, Some(selected), Message::Selected(2), &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Progress ───────────────────────────────────────────

    fn view_progress<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Progress").size(28),
            text("Displays an indicator showing the completion progress of a task.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            container(progress::styled(0.0..=100.0, self.progress_value, tokens)).width(400),
            text(format!("{}%", self.progress_value as u32)).size(13),
            separator::horizontal(tokens),
            text("Destructive").size(18),
            container(progress::destructive(0.0..=100.0, 80.0, tokens)).width(400),
            separator::horizontal(tokens),
            text("Success").size(18),
            container(progress::success(0.0..=100.0, 45.0, tokens)).width(400),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::progress;

// Default progress bar
let bar = progress::styled(0.0..=100.0, 60.0, &tokens);

// Destructive-styled bar
let bar_err = progress::destructive(0.0..=100.0, 30.0, &tokens);

// Success-styled bar
let bar_ok = progress::success(0.0..=100.0, 100.0, &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Sliders ────────────────────────────────────────────

    fn view_sliders<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Slider").size(28),
            text("An input where the user selects a value from within a given range.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            slider::styled(0.0..=1.0, self.slider_value, Message::SliderChanged, tokens).width(400),
            text(format!("Value: {:.2}", self.slider_value)).size(13),
            separator::horizontal(tokens),
            text("Controls progress bar")
                .size(14)
                .color(tokens.muted_foreground),
            container(progress::styled(0.0..=100.0, self.progress_value, tokens)).width(400),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::slider;

let s = slider::styled(0.0..=100.0, self.value, Message::SliderChanged, &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Alerts ─────────────────────────────────────────────

    fn view_alerts<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Alert").size(28),
            text("Displays a callout for important information.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Info").size(18),
            alert::info(
                "Heads up!",
                "You can add components to your app using the CLI.",
                tokens
            )
            .width(500),
            separator::horizontal(tokens),
            text("Error / Destructive").size(18),
            alert::error(
                "Error",
                "Your session has expired. Please log in again.",
                tokens,
            )
            .width(500),
            separator::horizontal(tokens),
            text("Success").size(18),
            alert::success(
                "Success!",
                "Your changes have been saved successfully.",
                tokens,
            )
            .width(500),
            separator::horizontal(tokens),
            text("Warning").size(18),
            alert::warning(
                "Warning",
                "This action cannot be undone. Proceed with caution.",
                tokens,
            )
            .width(500),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::alert::{self, AlertVariant};
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
);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Avatars ────────────────────────────────────────────

    fn view_avatars<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            text("Avatar").size(28),
            text("An image element with a fallback for representing the user.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Sizes").size(18),
            row![
                avatar::initials("John Doe", avatar::AvatarSize::Sm, tokens),
                avatar::initials("Jane Smith", avatar::AvatarSize::Default, tokens),
                avatar::initials("Bob Wilson", avatar::AvatarSize::Lg, tokens),
            ]
            .spacing(12)
            .align_y(iced::Center),
            separator::horizontal(tokens),
            text("Primary").size(18),
            row![
                avatar::initials_primary("AB", avatar::AvatarSize::Sm, tokens),
                avatar::initials_primary("CD", avatar::AvatarSize::Default, tokens),
                avatar::initials_primary("EF", avatar::AvatarSize::Lg, tokens),
            ]
            .spacing(12)
            .align_y(iced::Center),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::avatar::{self, AvatarSize};

// Default avatar with initials
let av = avatar::initials("John Doe", AvatarSize::Default, &tokens);

// Small avatar
let av_sm = avatar::initials("Jane", AvatarSize::Sm, &tokens);

// Large avatar
let av_lg = avatar::initials("Thang Nguyen", AvatarSize::Lg, &tokens);

// Primary-colored avatar
let av_primary = avatar::initials_primary("Floe", AvatarSize::Default, &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Selects ────────────────────────────────────────────

    fn view_selects<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        let options = vec![
            "Apple".to_string(),
            "Banana".to_string(),
            "Cherry".to_string(),
            "Grape".to_string(),
            "Mango".to_string(),
        ];

        column![
            text("Select").size(28),
            text("Displays a list of options for the user to pick from.")
                .size(14)
                .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Default").size(18),
            select::styled(
                options,
                self.select_value.as_ref(),
                Message::SelectChanged,
                tokens,
            )
            .width(240)
            .placeholder("Select a fruit…"),
            text(format!(
                "Selected: {}",
                self.select_value.as_deref().unwrap_or("None")
            ))
            .size(13)
            .color(tokens.muted_foreground),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::select;

let options = vec!["Apple", "Banana", "Cherry", "Mango"];
let sel = select::styled(
    options,
    self.selected_fruit.as_ref(),
    Message::FruitSelected,
    &tokens,
);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Icons ──────────────────────────────────────────────
    fn view_icons<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        let all_icons = icon::IconName::all();
        let display_count = 150.min(all_icons.len());
        let display_icons = &all_icons[0..display_count];

        let mut grid = column![].spacing(24);
        for chunk in display_icons.chunks(5) {
            let mut r = row![].spacing(32);
            for ic in chunk {
                r = r.push(tooltip::styled(
                    icon::view(*ic).size(28).color(tokens.foreground),
                    format!("{:?}", ic),
                    Position::Bottom,
                    tokens,
                ));
            }
            grid = grid.push(r);
        }

        column![
            typography::h2("Icons", tokens),
            typography::muted(
                format!(
                    "Built-in Lucide icons (showing {} of {}). Hover to see name.",
                    display_count,
                    all_icons.len()
                )
                .as_str(),
                tokens
            ),
            separator::horizontal(tokens),
            grid,
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Typography ─────────────────────────────────────────
    fn view_typography<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            typography::h2("Typography", tokens),
            typography::muted("Styles for headings, paragraphs, and other text elements.", tokens),
            separator::horizontal(tokens),
            typography::h1("Heading 1", tokens),
            typography::h2("Heading 2", tokens),
            typography::h3("Heading 3", tokens),
            typography::h4("Heading 4", tokens),
            typography::p("This is a standard paragraph. Typography is essential in any UI design to establish hierarchy and readability. The quick brown fox jumps over the lazy dog.", tokens),
            typography::lead("This is lead text. A bit larger and muted to serve as a subtitle.", tokens),
            typography::large("Large Text", tokens),
            typography::small("Small text for secondary details.", tokens),
            typography::muted("Muted text for captions or disabled items.", tokens),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(r#"use floe_ui::components::typography;

let h1 = typography::h1("Page Title", &tokens);
let h2 = typography::h2("Section", &tokens);
let h3 = typography::h3("Subsection", &tokens);
let h4 = typography::h4("Detail", &tokens);
let body = typography::p("Paragraph text.", &tokens);
let intro = typography::lead("Lead text for intros.", &tokens);
let big = typography::large("Large text.", &tokens);
let sm = typography::small("Small text.", &tokens);
let dim = typography::muted("Muted caption.", &tokens);"#, tokens),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Textareas ──────────────────────────────────────────
    fn view_textareas<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            typography::h2("Textarea", tokens),
            typography::muted(
                "Displays a form text area or a component that looks like a text area.",
                tokens
            ),
            separator::horizontal(tokens),
            typography::large("Default", tokens),
            textarea::styled(&self.textarea_content, Message::TextareaAction, tokens).height(150),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::textarea;
use iced::widget::text_editor;

// Create editor content in your state
let content = text_editor::Content::new();

// Create styled textarea
let editor = textarea::styled(&self.content, Message::EditorAction, &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Toggle Groups ──────────────────────────────────────
    fn view_toggle_groups<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            typography::h2("Toggle Group", tokens),
            typography::muted("A set of two-state buttons that can be toggled.", tokens),
            separator::horizontal(tokens),
            typography::large("Alignment", tokens),
            toggle_group::group(vec![
                toggle_group::item(
                    icon::view(icon::IconName::ChevronLeft).size(16),
                    self.toggle_group_value == "left",
                    toggle_group::ItemPosition::First,
                    Message::ToggleGroupChanged("left".to_string()),
                    tokens,
                ),
                toggle_group::item(
                    icon::view(icon::IconName::Minus).size(16),
                    self.toggle_group_value == "center",
                    toggle_group::ItemPosition::Middle,
                    Message::ToggleGroupChanged("center".to_string()),
                    tokens,
                ),
                toggle_group::item(
                    icon::view(icon::IconName::ChevronRight).size(16),
                    self.toggle_group_value == "right",
                    toggle_group::ItemPosition::Last,
                    Message::ToggleGroupChanged("right".to_string()),
                    tokens,
                ),
            ]),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::toggle_group::{self, ItemPosition};
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
]);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Tabs ───────────────────────────────────────────────
    fn view_tabs<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        let tab_list = tabs::list(
            vec![
                tabs::tab(
                    "Account",
                    self.active_tab == 0,
                    Message::TabChanged(0),
                    tokens,
                ),
                tabs::tab(
                    "Password",
                    self.active_tab == 1,
                    Message::TabChanged(1),
                    tokens,
                ),
                tabs::tab(
                    "Settings",
                    self.active_tab == 2,
                    Message::TabChanged(2),
                    tokens,
                ),
            ],
            tokens,
        );

        let tab_content: Element<'a, Message> = match self.active_tab {
            0 => typography::p(
                "Make changes to your account here. Click save when you're done.",
                tokens,
            )
            .into(),
            1 => typography::p(
                "Change your password here. After saving, you'll be logged out.",
                tokens,
            )
            .into(),
            _ => typography::p("Manage your application settings.", tokens).into(),
        };

        column![
            typography::h2("Tabs", tokens),
            typography::muted("A set of layered sections of content.", tokens),
            separator::horizontal(tokens),
            tab_list,
            card::styled(tab_content, tokens),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::tabs;

// Create a tab list with individual tabs
let tab_list = tabs::list(
    [
        tabs::tab("Account", self.active_tab == 0, Message::SetTab(0), &tokens),
        tabs::tab("Password", self.active_tab == 1, Message::SetTab(1), &tokens),
        tabs::tab("Settings", self.active_tab == 2, Message::SetTab(2), &tokens),
    ],
    &tokens,
);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Tables ─────────────────────────────────────────────
    fn view_tables<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        let filter_input = input::styled("Filter emails...", &self.input_value, tokens)
            .on_input(Message::InputChanged)
            .width(250.0);

        let columns_btn = iced::widget::button(
            row![
                text("Columns").size(14),
                icon::view(icon::IconName::ChevronDown).size(16)
            ]
            .spacing(8)
            .align_y(iced::Center),
        )
        .padding(button::ButtonSize::Default.padding())
        .style(button::outline_style(tokens))
        .on_press(Message::Noop);

        let toolbar = row![
            filter_input,
            iced::widget::Space::new().width(Fill),
            columns_btn
        ]
        .spacing(16)
        .align_y(iced::Center);

        let table_view = table::table(
            vec![
                container(
                    checkbox::styled("", self.table_rows_selected.iter().all(|&x| x), tokens)
                        .on_toggle(Message::TableSelectAll),
                )
                .width(Fixed(40.0))
                .into(),
                container(typography::muted("Status", tokens))
                    .width(FillPortion(2))
                    .into(),
                container(
                    row![
                        typography::muted("Email", tokens),
                        icon::view(icon::IconName::ArrowUpDown).size(14)
                    ]
                    .spacing(8)
                    .align_y(iced::Center),
                )
                .width(FillPortion(4))
                .into(),
                container(typography::muted("Amount", tokens))
                    .width(FillPortion(2))
                    .align_x(iced::alignment::Horizontal::Right)
                    .into(),
                container(iced::widget::Space::new().width(Fixed(40.0)))
                    .width(Fixed(40.0))
                    .into(),
            ],
            (0..4)
                .map(|i| {
                    let (status, email, amount) = match i {
                        0 => ("Success", "ken99@example.com", "$316.00"),
                        1 => ("Success", "abe45@example.com", "$242.00"),
                        2 => ("Processing", "monserrat44@example.com", "$837.00"),
                        _ => ("Failed", "carmella@example.com", "$0.00"),
                    };

                    let menu = floe_ui::components::dropdown::dropdown_menu(
                        column![
                            container(text("My Account").size(12).style(|_theme| text::Style {
                                color: Some(tokens.muted_foreground)
                            }))
                            .padding(iced::Padding::from([4, 8])),
                            floe_ui::components::dropdown::dropdown_item(
                                text("Profile"),
                                Message::Noop,
                                tokens
                            ),
                            floe_ui::components::dropdown::dropdown_item(
                                text("Billing"),
                                Message::Noop,
                                tokens
                            ),
                            floe_ui::components::dropdown::dropdown_item(
                                text("Settings"),
                                Message::Noop,
                                tokens
                            ),
                            separator::horizontal(tokens),
                            container(text("Team").size(12).style(|_theme| text::Style {
                                color: Some(tokens.muted_foreground)
                            }))
                            .padding(iced::Padding::from([4, 8])),
                            floe_ui::components::dropdown::dropdown_item(
                                text("New Team"),
                                Message::Noop,
                                tokens
                            ),
                            separator::horizontal(tokens),
                            floe_ui::components::dropdown::dropdown_item(
                                text("Log out"),
                                Message::Noop,
                                tokens
                            ),
                        ]
                        .spacing(2),
                        tokens,
                    )
                    .width(160.0);

                    let is_open = self.dropdown_open == Some(i);

                    row![
                        container(
                            checkbox::styled("", self.table_rows_selected[i], tokens)
                                .on_toggle(move |v| Message::TableRowSelected(i, v))
                        )
                        .width(Fixed(40.0)),
                        container(typography::p(status, tokens)).width(FillPortion(2)),
                        container(typography::p(email, tokens)).width(FillPortion(4)),
                        container(typography::p(amount, tokens))
                            .width(FillPortion(2))
                            .align_x(iced::alignment::Horizontal::Right),
                        container(
                            floe_ui::components::dropdown::Dropdown::new(
                                iced::widget::button(icon::view(icon::IconName::Ellipsis).size(16))
                                    .padding(8)
                                    .style(button::ghost_style(tokens))
                                    .on_press(if is_open {
                                        Message::ToggleDropdown(None)
                                    } else {
                                        Message::ToggleDropdown(Some(i))
                                    }),
                                menu,
                                is_open
                            )
                            .on_dismiss(Message::ToggleDropdown(None))
                        )
                        .width(Fixed(40.0))
                        .align_x(iced::alignment::Horizontal::Center),
                    ]
                })
                .collect::<Vec<_>>(),
            tokens,
        );

        let footer = row![
            typography::muted(
                &format!(
                    "{} of 4 row(s) selected.",
                    self.table_rows_selected.iter().filter(|&&x| x).count()
                ),
                tokens
            ),
            iced::widget::Space::new().width(Fill),
            button::outline("Previous", tokens).on_press(Message::Noop),
            button::outline("Next", tokens).on_press(Message::Noop)
        ]
        .spacing(8)
        .align_y(iced::Center);

        column![
            typography::h2("Data Table", tokens),
            typography::muted(
                "A complex data table with filtering, sorting, and pagination.",
                tokens
            ),
            separator::horizontal(tokens),
            toolbar,
            table_view,
            footer,
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::table;
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
);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Skeletons ──────────────────────────────────────────
    fn view_skeletons<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            typography::h2("Skeleton", tokens),
            typography::muted(
                "Use to show a placeholder while content is loading.",
                tokens
            ),
            separator::horizontal(tokens),
            row![
                skeleton::circle(48.0, tokens),
                column![
                    skeleton::rect(200.0, 16.0, tokens),
                    skeleton::rect(150.0, 16.0, tokens),
                ]
                .spacing(8),
            ]
            .spacing(16),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::skeleton;

// Rectangular placeholder
let rect = skeleton::rect(200, 16, &tokens);

// Circular placeholder (e.g. avatar loading)
let circle = skeleton::circle(48, &tokens);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }

    // ── Section: Pagination ─────────────────────────────────────────
    fn view_pagination<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        column![
            typography::h2("Pagination", tokens),
            typography::muted(
                "Pagination with page navigation, next and previous links.",
                tokens
            ),
            separator::horizontal(tokens),
            pagination::pagination(vec![
                pagination::previous(Some(Message::Noop), tokens),
                pagination::item("1", true, Message::Noop, tokens),
                pagination::item("2", false, Message::Noop, tokens),
                pagination::item("3", false, Message::Noop, tokens),
                pagination::ellipsis(tokens),
                pagination::next(Some(Message::Noop), tokens),
            ]),
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(
                r#"use floe_ui::components::pagination;

let nav = pagination::pagination([
    pagination::previous(Some(Message::PrevPage), &tokens),
    pagination::item("1", true, Message::GoToPage(1), &tokens),
    pagination::item("2", false, Message::GoToPage(2), &tokens),
    pagination::item("3", false, Message::GoToPage(3), &tokens),
    pagination::ellipsis(&tokens),
    pagination::next(Some(Message::NextPage), &tokens),
]);"#,
                tokens
            ),
        ]
        .spacing(16)
        .into()
    }
    // ── Section: Dropdowns ──────────────────────────────────────────
    fn view_dropdowns<'a>(&'a self, tokens: &'a DesignTokens) -> Element<'a, Message> {
        let shortcut = |label: &'static str, hotkey: &'static str| {
            row![
                text(label),
                iced::widget::Space::new().width(Fill),
                text(hotkey)
                    .size(12)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(tokens.muted_foreground)
                    })
            ]
            .spacing(16)
            .align_y(iced::alignment::Vertical::Center)
        };

        let menu = floe_ui::components::dropdown::dropdown_menu(
            column![
                container(
                    text("My Account")
                        .size(12)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(tokens.muted_foreground)
                        })
                )
                .padding(iced::Padding::from([4, 8])),
                floe_ui::components::dropdown::dropdown_item(
                    shortcut("Profile", "⇧⌘P"),
                    Message::Noop,
                    tokens
                ),
                floe_ui::components::dropdown::dropdown_item(
                    shortcut("Billing", "⌘B"),
                    Message::Noop,
                    tokens
                ),
                floe_ui::components::dropdown::dropdown_item(
                    shortcut("Settings", "⌘S"),
                    Message::Noop,
                    tokens
                ),
                separator::horizontal(tokens),
                container(
                    text("Team")
                        .size(12)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(tokens.muted_foreground)
                        })
                )
                .padding(iced::Padding::from([4, 8])),
                floe_ui::components::dropdown::dropdown_item(
                    shortcut("Invite users", ">"),
                    Message::Noop,
                    tokens
                ),
                floe_ui::components::dropdown::dropdown_item(
                    shortcut("New Team", "⌘+T"),
                    Message::Noop,
                    tokens
                ),
                separator::horizontal(tokens),
                floe_ui::components::dropdown::dropdown_item(text("GitHub"), Message::Noop, tokens),
                floe_ui::components::dropdown::dropdown_item(
                    text("Support"),
                    Message::Noop,
                    tokens
                ),
                container(text("API").style(|_theme| iced::widget::text::Style {
                    color: Some(tokens.muted_foreground)
                }))
                .width(Fill)
                .padding(iced::Padding::from([6, 8])),
                separator::horizontal(tokens),
                floe_ui::components::dropdown::dropdown_item(
                    shortcut("Log out", "⇧⌘Q"),
                    Message::Noop,
                    tokens
                ),
            ]
            .spacing(2),
            tokens,
        )
        .width(220.0);

        let dropdown = floe_ui::components::dropdown::Dropdown::new(
            button::outline("Open", tokens).on_press(Message::ToggleShowcaseDropdown(
                !self.showcase_dropdown_open,
            )),
            menu,
            self.showcase_dropdown_open,
        )
        .on_dismiss(Message::ToggleShowcaseDropdown(false));

        column![
            typography::h2("Dropdown Menu", tokens),
            typography::muted("Displays a menu to the user — such as a set of actions or functions — triggered by a button.", tokens),
            separator::horizontal(tokens),
            container(dropdown)
                .width(Fill)
                .height(Fixed(400.0)), // Give it enough height so the dropdown doesn't overflow out of the gallery
            separator::horizontal(tokens),
            text("Usage").size(18),
            view_usage(r#"use floe_ui::components::{dropdown, button, separator};
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
).on_dismiss(Message::CloseMenu);"#, tokens),
        ]
        .spacing(16)
        .into()
    }
}
