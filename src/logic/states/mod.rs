use crate::prelude::*;

pub mod app_state;
pub mod game_state;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                AppStatePlugin,
                GameStatePlugin
            ));
    }
}
