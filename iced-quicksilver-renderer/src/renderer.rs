use crate::{Defaults, Primitive};

#[cfg(any(feature = "image", feature = "svg"))]
use crate::image::{self, Image};

use iced_native::{self, MouseCursor};

use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background, Color},
    lifecycle::Window,
};

mod widget;

/// A [`quicksilver`] renderer for `[iced]`.
///
/// [`iced`]: https://github.com/hecrj/iced
/// [`quicksilver`]: https://github.com/ryanisaacg/quicksilver
#[derive(Debug)]
pub struct Renderer {}

impl Renderer {
    /// Creates a new [`Renderer`].
    ///
    /// [`Renderer`]: struct.Renderer.html
    pub fn new() -> Self {
        Renderer {}
    }

    /// Draws the provided primitives in the given [`Target`].
    ///
    /// The text provided as overlay will be renderer on top of the primitives.
    /// This is useful for rendering debug information.
    ///
    /// [`Target`]: struct.Target.html
    pub fn draw<T: AsRef<str>>(
        &mut self,
        window: &mut Window,
        (primitive, mouse_cursor): &(Primitive, MouseCursor),
        overlay: &[T],
    ) -> MouseCursor {
        self.draw_primitive(
            window, // Vector::new(0.0, 0.0),
            primitive,
        );

        *mouse_cursor
    }

    fn draw_primitive<'a>(
        &mut self,
        window: &mut Window,
        // translation: Vector,
        primitive: &'a Primitive,
    ) {
        match primitive {
            Primitive::None => {}
            Primitive::Group { primitives } => {
                // TODO: Inspect a bit and regroup (?)
                for primitive in primitives {
                    self.draw_primitive(window, primitive)
                }
            }
            // Primitive::Text {
            //     content,
            //     center,
            //     color,
            //     size,
            //     font,
            //     image_cache,
            // } => {}
            Primitive::Quad {
                bounds: bounds,
                background,
                border_width,
                border_color,
            } => {
                let bounds = iced_rect_to_qs(bounds);
                if *border_width > 0 {
                    window.draw(
                        &Rectangle {
                            pos: bounds.pos - Vector::ONE * *border_width,
                            size: bounds.size + Vector::ONE * (*border_width * 2),
                        },
                        Background::Col(iced_col_to_qs(border_color)),
                    );
                }
                let background_color = match background {
                    iced_native::Background::Color(col) => col,
                };
                window.draw(&bounds, Background::Col(iced_col_to_qs(background_color)));
            }
        }
    }
}

impl iced_native::Renderer for Renderer {
    type Output = (Primitive, MouseCursor);
    type Defaults = Defaults;

    fn layout<'a, Message>(
        &mut self,
        element: &iced_native::Element<'a, Message, Self>,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        let node = element.layout(self, limits);

        node
    }
}

fn iced_rect_to_qs(rect: &iced_native::Rectangle) -> Rectangle {
    Rectangle {
        pos: Vector::new(rect.x, rect.y),
        size: Vector::new(rect.width, rect.height),
    }
}

fn iced_col_to_qs(col: &iced_native::Color) -> Color {
    let &iced_native::Color { r, g, b, a } = col;
    Color { r, g, b, a }
}
