use std::sync::Arc;

use iced_native::{image, Background, Color, Font, Image, Rectangle, Vector};

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
    // /// A text primitive
    // Text {
    //     /// The contents of the text
    //     content: String,
    //     /// The center of the text
    //     center: Vector,
    //     /// The color of the text
    //     color: Color,
    //     /// The size of the text
    //     size: f32,
    //     /// The font of the text
    //     font: Font,
    //     /// A cache of the image of the text
    //     image_cache: Option<image::Handle>,
    // },
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

impl Default for Primitive {
    fn default() -> Primitive {
        Primitive::None
    }
}
