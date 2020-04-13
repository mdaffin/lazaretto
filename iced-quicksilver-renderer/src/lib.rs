//! A [`wgpu`] renderer for [`iced_native`].
//!
//! ![`iced_wgpu` crate graph](https://github.com/hecrj/iced/blob/cae26cb7bc627f4a5b3bcf1cd023a0c552e8c65e/docs/graphs/wgpu.png?raw=true)
//!
//! For now, it is the default renderer of [Iced] in native platforms.
//!
//! [`wgpu`] supports most modern graphics backends: Vulkan, Metal, DX11, and
//! DX12 (OpenGL and WebGL are still WIP). Additionally, it will support the
//! incoming [WebGPU API].
//!
//! Currently, `iced_wgpu` supports the following primitives:
//! - Text, which is rendered using [`wgpu_glyph`]. No shaping at all.
//! - Quads or rectangles, with rounded borders and a solid background color.
//! - Clip areas, useful to implement scrollables or hide overflowing content.
//! - Images and SVG, loaded from memory or the file system.
//! - Meshes of triangles, useful to draw geometry freely.
//!
//! [Iced]: https://github.com/hecrj/iced
//! [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
//! [`wgpu`]: https://github.com/gfx-rs/wgpu-rs
//! [WebGPU API]: https://gpuweb.github.io/gpuweb/
//! [`wgpu_glyph`]: https://github.com/hecrj/wgpu_glyph
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_results)]
#![forbid(unsafe_code)]
#![forbid(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[macro_use]
extern crate derivative;

pub mod defaults;
// pub mod settings;
// pub mod triangle;
pub mod widget;
// pub mod window;

mod primitive;
// mod quad;
mod renderer;
// mod target;
// mod text;
// mod transformation;
// mod viewport;

// pub use wgpu;

pub use defaults::Defaults;
pub use primitive::Primitive;
pub use renderer::Renderer;
// pub use settings::Settings;
// pub use target::Target;
// pub use viewport::Viewport;

#[doc(no_inline)]
pub use widget::*;

// pub(crate) use quad::Quad;
// pub(crate) use transformation::Transformation;

// #[cfg(any(feature = "image", feature = "svg"))]
// mod image;
