use crate::prelude::*;

pub mod button_input;
pub mod keyboard_input_handler;
pub mod mouse_input_handler;
pub mod move_request;
pub mod keyboard_utilities;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            KeyboardInputHandlerPlugin,
            MouseInputHandlerPlugin,
            ButtonInputPlugin,
        ));
    }
}
