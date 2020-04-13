//! Convert [`winit`] types into [`iced_native`] types, and viceversa.
//!
//! [`winit`]: https://github.com/rust-windowing/winit
//! [`iced_native`]: https://github.com/hecrj/iced/tree/master/native
use crate::{
    input::{
        keyboard::{self, KeyCode, ModifiersState},
        mouse, ButtonState,
    },
    window, Event, Mode, MouseCursor,
};
