use crate::prelude::*;

pub mod start_button_and_above_text_logic;
pub mod game_screen_text_logic;
pub mod pop_up_message_logic;


pub struct MessagesGraphicsPlugin;

impl Plugin for MessagesGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((
                TextAboveStartButtonLogicPlugin,
                GameScreenTextLogicPlugin,
                PopUpMessageLogicPlugin
            ))
        ;
    }
}