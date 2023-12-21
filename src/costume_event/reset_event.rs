use bevy::prelude::*;


#[derive (Event, Default)]
pub struct ResetBoardGraphics;

#[derive (Event, Default)]
pub struct ResetBoardLogic{
    pub reroll_solved: bool
}

pub struct ResetEventPlugin;

impl Plugin for ResetEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ResetBoardLogic>()
            .add_event::<ResetBoardGraphics>()
            ;
    }
}