use bevy::prelude::*;

#[derive (Event, Default)]
pub struct ResetBoardGraphics;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResetBoardGraphics>()
            ;
    }
}