use crate::prelude::*;

#[derive(Event)]
pub struct EndGame;

#[derive(Event, Default)]
pub struct ToggleMenu{
    pub out_of_menu_into: Option<AppState>   
}

pub struct AppEventPlugin;

impl Plugin for AppEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<EndGame>()
            .add_event::<ToggleMenu>();
    }
}
