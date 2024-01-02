use crate::prelude::*;

pub mod mouse_input_handler;
pub mod keyboard_input_handler;
pub mod button_input;
pub mod move_request;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                KeyboardInputHandlerPlugin, 
                MouseInputHandlerPlugin,
                ButtonInputPlugin
            ))
            ;
    }
}