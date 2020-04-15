use std::{cell::RefCell, collections::HashMap};

use quicksilver::{
    geom::Vector,
    graphics::{Color, Font, FontStyle, Image},
};

pub const BUILTIN_ICONS: iced_native::Font = iced_native::Font::External {
    name: "iced_wgpu icons",
    bytes: include_bytes!("text/icons.ttf"),
};

pub const CHECKMARK_ICON: char = '\u{F00C}';

const FALLBACK_FONT: &[u8] = include_bytes!("../fonts/Lato-Regular.ttf");

#[derive(Copy, Clone, Debug)]
struct FontId(usize);

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Pipeline {
    font_map: RefCell<HashMap<String, FontId>>,
    #[derivative(Debug = "ignore")]
    fonts: RefCell<Vec<Font>>,
}

impl Pipeline {
    pub fn new(default_font: Option<&'static [u8]>) -> Self {
        // TODO: System font (?), font customization

        let default_font = default_font
            .map(Font::from_slice)
            .and_then(Result::ok)
            .unwrap_or_else(|| Font::from_slice(FALLBACK_FONT).expect("loading fallback font"));

        let fonts = RefCell::new(vec![default_font]);

        Pipeline {
            font_map: RefCell::new(HashMap::new()),
            fonts,
        }
    }

    pub fn default_font(&self) -> FontId {
        FontId(0)
    }

    pub fn to_image(
        &self,
        content: &str,
        color: Color,
        size: f32,
        font: iced_native::Font,
    ) -> Image {
        let FontId(font_index) = self.find_font(font);
        let font = &self.fonts.borrow()[font_index];
        font.render(&content, &FontStyle::new(size, color))
            .expect("failed to render text")
    }

    pub fn find_font(&self, font: iced_native::Font) -> FontId {
        match font {
            iced_native::Font::Default => FontId(0),
            iced_native::Font::External { name, bytes } => {
                if let Some(font_id) = self.font_map.borrow().get(name) {
                    return *font_id;
                }

                let font = Font::from_slice(bytes).expect("failed to load font");

                // one borrow -> atomic
                let mut fonts = self.fonts.borrow_mut();
                let font_id = FontId(fonts.len());
                fonts.push(font);

                let _ = self
                    .font_map
                    .borrow_mut()
                    .insert(String::from(name), font_id);

                font_id
            }
        }
    }
}
