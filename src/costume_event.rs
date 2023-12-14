use bevy::prelude::*;

#[derive (Event, Default)]
pub struct ResetBoardGraphics;

#[derive (Event, Default)]
pub struct ResetBoardLogic;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResetBoardLogic>()
            .add_event::<ResetBoardGraphics>()
            ;
    }
}