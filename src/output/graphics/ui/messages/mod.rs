pub mod victory_message;

pub mod text_above_start_button;

use crate::prelude::*;

pub struct MessagesGraphicsPlugin;

impl Plugin for MessagesGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((
				VictoryMessagePlugin,
                TextAboveStartButtonPlugin
            ));
    }
}