pub mod victory_message;
pub mod are_you_sure_message_spawner;

use crate::prelude::*;

pub struct MessagesSpawnersPlugin;

impl Plugin for MessagesSpawnersPlugin {
    fn build(&self, app: &mut App) {
        app
			.add_plugins((
                VictoryMessagePlugin,
                AreYouSureMessageSpawnerPlugin
            ))
        ;
    }
}