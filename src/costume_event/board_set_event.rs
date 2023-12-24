use bevy::prelude::*;


#[derive (Event, Default)]
pub struct MoveExistingTilesGraphics;

#[derive (Event, Default)]
pub struct ConductANewBoardBuilding{
    pub reroll_solved: bool
}

#[derive (Event, Default)]
pub struct BuildNewBoard{
    pub reroll_solved: bool
}

pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BuildNewBoard>()
            .add_event::<MoveExistingTilesGraphics>()
            ;
    }
}