pub mod victory_message;

pub mod start_button_and_above_text;
pub mod save_button_and_above_text;

use crate::prelude::*;

pub struct MessagesGraphicsPlugin;

impl Plugin for MessagesGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((
				VictoryMessagePlugin,
                TextAboveStartButtonPlugin,
                TextAboveSaveButtonPlugin
            ))
        ;
    }
}