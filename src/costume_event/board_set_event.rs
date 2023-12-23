use bevy::prelude::*;


#[derive (Event, Default)]
pub struct MoveExistingTilesGraphics;

#[derive (Event, Default)]
pub struct ResetBoardWithCurrentSettings{
    pub reroll_solved: bool
}

#[derive (Event, Default)]
pub struct SpawnBoardWithNewSettings;


pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResetBoardWithCurrentSettings>()
            .add_event::<MoveExistingTilesGraphics>()
            .add_event::<SpawnBoardWithNewSettings>()
            ;
    }
}