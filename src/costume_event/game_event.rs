use crate::prelude::*;

#[derive(Event)]
pub struct ToggleVictoryMessage;

pub struct GameEventPlugin;

impl Plugin for GameEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ToggleVictoryMessage>();
    }
}