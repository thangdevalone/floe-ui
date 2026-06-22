use crate::theme::tokens::DesignTokens;
use iced::advanced::{
    layout::{self, Layout, Node},
    overlay, renderer,
    widget::{tree, Tree, Widget},
    Clipboard, Shell,
};
use iced::event::Event;
use iced::mouse;
use iced::{Element, Length, Point, Rectangle, Size, Vector, Theme, Border, Shadow, Color, Background, Padding};
use iced::widget::{container, button, Container, Button};
use iced::widget::button::Status;

/// A custom widget that displays a base element and, when opened, an overlay menu.
pub struct Dropdown<'a, Message, Theme, Renderer> {
    base: Element<'a, Message, Theme, Renderer>,
    menu: Element<'a, Message, Theme, Renderer>,
    is_open: bool,
    on_dismiss: Option<Message>,
}

impl<'a, Message, Theme, Renderer> Dropdown<'a, Message, Theme, Renderer> {
    pub fn new(
        base: impl Into<Element<'a, Message, Theme, Renderer>>,
        menu: impl Into<Element<'a, Message, Theme, Renderer>>,
        is_open: bool,
    ) -> Self {
        Self {
            base: base.into(),
            menu: menu.into(),
            is_open,
            on_dismiss: None,
        }
    }

    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Dropdown<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn size(&self) -> Size<Length> {
        self.base.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> Node {
        self.base.as_widget_mut().layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.base.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<()>()
    }

    fn state(&self) -> tree::State {
        tree::State::None
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.base), Tree::new(&self.menu)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.base, &self.menu]);
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation,
    ) {
        self.base.as_widget_mut().operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.base.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.base.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        if !self.is_open {
            return None;
        }

        let bounds = layout.bounds();
        let target = Rectangle {
            x: bounds.x + translation.x,
            y: bounds.y + translation.y,
            ..bounds
        };

        Some(overlay::Element::new(Box::new(DropdownOverlay {
            state: &mut tree.children[1],
            content: &mut self.menu,
            target,
            on_dismiss: self.on_dismiss.clone(),
        })))
    }
}

impl<'a, Message, Theme: 'a, Renderer> From<Dropdown<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(dropdown: Dropdown<'a, Message, Theme, Renderer>) -> Self {
        Self::new(dropdown)
    }
}

struct DropdownOverlay<'a, 'b, Message, Theme, Renderer> {
    state: &'b mut Tree,
    content: &'b mut Element<'a, Message, Theme, Renderer>,
    target: Rectangle,
    on_dismiss: Option<Message>,
}

impl<'a, 'b, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for DropdownOverlay<'a, 'b, Message, Theme, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> Node {
        let limits = layout::Limits::new(Size::ZERO, bounds)
            .width(Length::Shrink)
            .height(Length::Shrink);

        let mut node = self.content.as_widget_mut().layout(self.state, renderer, &limits);

        let mut y = self.target.y + self.target.height + 4.0;
        let mut x = self.target.x;
        
        // Prevent overflow
        if y + node.bounds().height > bounds.height {
            y = self.target.y - node.bounds().height - 4.0;
        }
        if x + node.bounds().width > bounds.width {
            x = bounds.width - node.bounds().width - 4.0;
        }

        node.move_to_mut(Point::new(x.max(0.0), y.max(0.0)));
        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.content.as_widget().draw(
            self.state,
            renderer,
            theme,
            style,
            layout,
            cursor,
            &layout.bounds(),
        );
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let is_clicked_outside = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
                let cursor_position = cursor.position().unwrap_or(Point::ORIGIN);
                !layout.bounds().contains(cursor_position)
            }
            _ => false,
        };

        self.content.as_widget_mut().update(
            self.state,
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        );

        // TODO: In Iced 0.14, update doesn't return Status. 
        // We can just publish the dismiss event if clicked outside.
        if is_clicked_outside {
            if let Some(on_dismiss) = &self.on_dismiss {
                shell.publish(on_dismiss.clone());
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.state,
            layout,
            cursor,
            &layout.bounds(),
            renderer,
        )
    }
}

// ── Styling Helpers ─────────────────────────────────────────────────────

/// Creates a styled container for the dropdown menu card.
pub fn menu_style<'a>(tokens: &'a DesignTokens) -> impl Fn(&Theme) -> container::Style + 'a {
    move |_| container::Style {
        text_color: Some(tokens.popover_foreground),
        background: Some(Background::Color(tokens.popover)),
        border: Border {
            color: tokens.border,
            width: 1.0,
            radius: tokens.radius_md.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.15),
            offset: Vector::new(0.0, 4.0),
            blur_radius: 12.0,
        },
        ..Default::default()
    }
}

/// Creates a styling function for dropdown menu items (buttons).
pub fn item_style<'a>(tokens: &'a DesignTokens) -> impl Fn(&Theme, Status) -> button::Style + 'a {
    move |_theme, status| {
        let base = button::Style {
            background: None,
            text_color: tokens.popover_foreground,
            border: Border::default(),
            shadow: Shadow::default(),
            ..button::Style::default()
        };

        match status {
            Status::Active => base,
            Status::Hovered => button::Style {
                background: Some(Background::Color(tokens.muted)),
                text_color: tokens.accent_foreground,
                ..base
            },
            Status::Pressed => button::Style {
                background: Some(Background::Color(tokens.accent)),
                text_color: tokens.accent_foreground,
                ..base
            },
            Status::Disabled => button::Style {
                text_color: tokens.muted_foreground,
                ..base
            },
        }
    }
}

/// A pre-configured column for dropdown content with padding.
pub fn dropdown_menu<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    tokens: &'a DesignTokens,
) -> Container<'a, Message> {
    container(content.into())
        .padding(4)
        .style(menu_style(tokens))
}

/// A pre-configured button representing a dropdown item.
pub fn dropdown_item<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    on_press: Message,
    tokens: &'a DesignTokens,
) -> Button<'a, Message> {
    button(
        container(content)
            .width(Length::Fill)
            .padding(Padding::from([6, 8]))
    )
    .width(Length::Fill)
    .style(item_style(tokens))
    .on_press(on_press)
}

