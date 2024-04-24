use crate::prelude::*;

pub mod start_button_and_above_text_logic;
pub mod save_button_and_above_text_logic;
pub mod are_you_sure_message_logic;


pub struct MessagesGraphicsPlugin;

impl Plugin for MessagesGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((
                TextAboveStartButtonLogicPlugin,
                TextAboveSaveButtonLogicPlugin,
                AreYouSureMessageLogicPlugin
            ))
        ;
    }
}