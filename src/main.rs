mod controls;
mod scene;

use controls::Controls;
use scene::Scene;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Color, View},
    input::ButtonState as QsButtonState,
    input::MouseButton as QsMouseButton,
    input::MouseCursor as QsMouseCursor,
    lifecycle::{run, Event as QsEvent, Settings, State, Window},
    Result,
};

use iced_native::{
    input::{
        keyboard::Event as IcedKeyboardEvent, mouse::Button as IcedButton,
        mouse::Event as IcedMouseEvent, mouse::ScrollDelta as IcedScrollDelta,
        ButtonState as IcedButtonState,
    },
    Cache, Event as IcedEvent, MouseCursor as IcedMouseCursor, Size, UserInterface,
};

use iced_quicksilver_renderer::{Primitive, Renderer};

// A unit struct that we're going to use to run the Quicksilver functions
// If we wanted to store persistent state, we would put it in here.
struct Application {
    events: Vec<IcedEvent>,
    cache: Option<Cache>,
    renderer: Renderer,
    output: (Primitive, IcedMouseCursor),
    scene: Scene,
    controls: Controls,
}

impl State for Application {
    // Initialize the struct
    fn new() -> Result<Application> {
        env_logger::init();
        let mut resized = false;

        // Initialize iced
        let mut events = Vec::new();
        let mut cache = Some(Cache::default());
        let mut renderer = Renderer::new();
        let mut output = (Primitive::None, IcedMouseCursor::OutOfBounds);

        // Initialize scene and GUI controls
        let mut scene = Scene::new();
        let mut controls = Controls::new();

        Ok(Application {
            events,
            cache,
            renderer,
            output,
            scene,
            controls,
        })
    }

    fn event(&mut self, event: &QsEvent, window: &mut Window) -> Result<()> {
        if let Some(iced_event) = convert_event(event) {
            self.events.push(iced_event);
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        // We need to:
        // 1. Process events of our user interface.
        // 2. Update state as a result of any interaction.
        // 3. Generate a new output for our renderer.

        let logical_size = window.screen_size();
        window.set_view(View::new(Rectangle::new_sized(logical_size)));

        // First, we build our user interface.
        let mut user_interface = UserInterface::build(
            self.controls.view(&self.scene),
            Size::new(logical_size.x, logical_size.y),
            self.cache.take().unwrap(),
            &mut self.renderer,
        );

        // Then, we process the events, obtaining messages in return.
        let messages = user_interface.update(self.events.drain(..), None, &self.renderer);

        let user_interface = if messages.is_empty() {
            // If there are no messages, no interactions we care about have
            // happened. We can simply leave our user interface as it is.
            user_interface
        } else {
            // If there are messages, we need to update our state
            // accordingly and rebuild our user interface.
            // We can only do this if we drop our user interface first
            // by turning it into its cache.
            self.cache = Some(user_interface.into_cache());

            // In this example, `Controls` is the only part that cares
            // about messages, so updating our state is pretty
            // straightforward.
            for message in messages {
                self.controls.update(message, &mut self.scene);
            }

            // Once the state has been changed, we rebuild our updated
            // user interface.
            UserInterface::build(
                self.controls.view(&self.scene),
                Size::new(logical_size.x, logical_size.y),
                self.cache.take().unwrap(),
                &mut self.renderer,
            )
        };

        // Finally, we just need to draw a new output for our renderer,
        self.output = user_interface.draw(&mut self.renderer);

        // update our cache,
        self.cache = Some(user_interface.into_cache());

        // We draw the scene first
        self.scene.draw(window)?;

        // And then iced on top
        let mouse_cursor = self
            .renderer
            .draw(window, &self.output, &Vec::<String>::new());

        // And update the mouse cursor
        window.set_cursor(convert_mouse_cursor(mouse_cursor));

        // We completed with no errors
        Ok(())
    }
}

// The main isn't that important in Quicksilver: it just serves as an entrypoint into the event
// loop
fn main() {
    // Run with Application as the event handler, with a window title of 'Draw Geometry' and a
    // size of (800, 600)
    run::<Application>("Draw Geometry", Vector::new(800, 600), Settings::default());
}

fn convert_event(event: &QsEvent) -> Option<IcedEvent> {
    match event {
        &QsEvent::Typed(c) => Some(IcedEvent::Keyboard(IcedKeyboardEvent::CharacterReceived(c))),
        &QsEvent::MouseMoved(Vector { x, y }) => {
            Some(IcedEvent::Mouse(IcedMouseEvent::CursorMoved { x, y }))
        }
        &QsEvent::MouseEntered => Some(IcedEvent::Mouse(IcedMouseEvent::CursorEntered)),
        &QsEvent::MouseExited => Some(IcedEvent::Mouse(IcedMouseEvent::CursorLeft)),
        &QsEvent::MouseWheel(Vector { x, y }) => {
            Some(IcedEvent::Mouse(IcedMouseEvent::WheelScrolled {
                delta: IcedScrollDelta::Pixels { x, y },
            }))
        }
        &QsEvent::MouseButton(button, state) => {
            Some(IcedEvent::Mouse(convert_button_state(button, state)?))
        }
        _ => None,
    }
}

fn convert_button_state(button: QsMouseButton, state: QsButtonState) -> Option<IcedMouseEvent> {
    Some(IcedMouseEvent::Input {
        state: match state {
            QsButtonState::Pressed => IcedButtonState::Pressed,
            QsButtonState::Released => IcedButtonState::Released,
            QsButtonState::Held => None?,
            QsButtonState::NotPressed => None?,
        },
        button: match button {
            QsMouseButton::Left => IcedButton::Left,
            QsMouseButton::Right => IcedButton::Right,
            QsMouseButton::Middle => IcedButton::Middle,
        },
    })
}

pub fn convert_mouse_cursor(mouse_cursor: IcedMouseCursor) -> QsMouseCursor {
    match mouse_cursor {
        IcedMouseCursor::OutOfBounds => QsMouseCursor::Default,
        IcedMouseCursor::Idle => QsMouseCursor::Default,
        IcedMouseCursor::Pointer => QsMouseCursor::Hand,
        IcedMouseCursor::Working => QsMouseCursor::Progress,
        IcedMouseCursor::Grab => QsMouseCursor::Grab,
        IcedMouseCursor::Grabbing => QsMouseCursor::Grabbing,
        IcedMouseCursor::Text => QsMouseCursor::Text,
        IcedMouseCursor::ResizingHorizontally => QsMouseCursor::EwResize,
        IcedMouseCursor::ResizingVertically => QsMouseCursor::NsResize,
    }
}
