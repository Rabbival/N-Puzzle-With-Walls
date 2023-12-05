use crate::{prelude::*, logic::board_manager};

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, move_tiles_with_keyboard);
    }
}

fn move_tiles_with_keyboard(keyboard_input: Res<Input<KeyCode>>){
    if keyboard_input.just_pressed(KeyCode::W) ||  keyboard_input.just_pressed(KeyCode::Up){
        info!("up just pressed");
    }
}