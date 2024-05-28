pub mod victory_message;
pub mod pop_up_message;
pub mod confirm_allowed;

use crate::prelude::*;

pub struct MessagesSpawnersPlugin;

impl Plugin for MessagesSpawnersPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((
                VictoryMessagePlugin,
                PopUpMessagePlugin
            ))
        ;
    }
}