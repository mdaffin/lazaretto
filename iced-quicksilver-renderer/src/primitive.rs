use std::sync::Arc;

use iced_native::{
    image, svg, Background, Color, Font, HorizontalAlignment, Point, Rectangle, Vector,
    VerticalAlignment,
};

/// A rendering primitive.
#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub enum Primitive {
    /// An empty primitive
    None,
    /// A group of primitives
    Group {
        /// The primitives of the group
        primitives: Vec<Primitive>,
    },
    /// A text primitive
    Text {
        /// The contents of the text
        content: String,
        /// The bounds of the text
        bounds: Rectangle,
        /// The color of the text
        color: Color,
        /// The size of the text
        size: f32,
        /// The font of the text
        #[derivative(Debug(format_with = "debug_font_name"))]
        font: Font,
        /// The horizontal alignment of the text
        horizontal_alignment: HorizontalAlignment,
        /// The vertical alignment of the text
        vertical_alignment: VerticalAlignment,
    },
    /// A quad primitive
    Quad {
        /// The bounds of the quad
        bounds: Rectangle,
        /// The background of the quad
        // TODO: when `iced` gets non-plain backgrounds,
        // use Background and deal with the lifetime+formatting issues
        background: Background,
        /// The border width of the quad
        border_width: u16,
        /// The border color of the quad
        border_color: Color,
    },
}

fn debug_font_name(
    font: &Font,
    fmt: &mut std::fmt::Formatter<'_>,
) -> std::result::Result<(), std::fmt::Error> {
    match font {
        Font::Default => fmt.pad("<Default>"),
        Font::External { name, .. } => fmt.pad(&format!("{:?}", name)),
    }
}

impl Default for Primitive {
    fn default() -> Primitive {
        Primitive::None
    }
}
