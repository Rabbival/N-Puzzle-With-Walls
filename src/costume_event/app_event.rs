use crate::prelude::*;

#[derive (Event)]
pub struct EndGame;

#[derive (Event)]
pub struct ToggleMenu;


pub struct AppEventPlugin;

impl Plugin for AppEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EndGame>()
            .add_event::<ToggleMenu>()
            ;
    }
}