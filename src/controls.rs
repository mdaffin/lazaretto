use crate::Scene;

use iced_quicksilver_renderer::Renderer;

use quicksilver::graphics::Color as QsColor;

use iced_native::{slider, Align, Color, Column, Element, Length, Row, Slider, Text};

pub struct Controls {
    sliders: [slider::State; 3],
}

#[derive(Debug)]
pub enum Message {
    BackgroundColorChanged(QsColor),
}

impl Controls {
    pub fn new() -> Controls {
        Controls {
            sliders: Default::default(),
        }
    }

    pub fn update(&self, message: Message, scene: &mut Scene) {
        match message {
            Message::BackgroundColorChanged(color) => {
                scene.background_color = color;
            }
        }
    }

    pub fn view(&mut self, scene: &Scene) -> Element<Message, Renderer> {
        let [r, g, b] = &mut self.sliders;
        let background_color = scene.background_color;

        let sliders = Row::new()
            .width(Length::Units(500))
            .spacing(20)
            .push(Slider::new(
                r,
                0.0..=1.0,
                scene.background_color.r,
                move |r| {
                    Message::BackgroundColorChanged(QsColor {
                        r,
                        ..background_color
                    })
                },
            ))
            .push(Slider::new(
                g,
                0.0..=1.0,
                scene.background_color.g,
                move |g| {
                    Message::BackgroundColorChanged(QsColor {
                        g,
                        ..background_color
                    })
                },
            ))
            .push(Slider::new(
                b,
                0.0..=1.0,
                scene.background_color.b,
                move |b| {
                    Message::BackgroundColorChanged(QsColor {
                        b,
                        ..background_color
                    })
                },
            ));

        Row::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Align::End)
            .push(
                Column::new()
                    .width(Length::Fill)
                    .align_items(Align::End)
                    .push(
                        Column::new()
                            .padding(10)
                            .spacing(10)
                            // .push(Text::new("Background color").color(Color::WHITE))
                            .push(sliders)
                            // .push(
                            //     Text::new(format!("{:?}", background_color))
                            //         .size(14)
                            //         .color(Color::WHITE),
                            // ),
                    ),
            )
            .into()
    }
}
