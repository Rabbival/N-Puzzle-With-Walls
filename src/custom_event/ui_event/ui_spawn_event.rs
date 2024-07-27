use crate::prelude::*;


#[derive(Event, Default)]
pub struct SpawnTextsAndButtons {
    pub board_props_button_style: Style,
    pub big_button_style: Style,
    pub space_bar_looking_button_style: Style,
    pub common_button_style: Style,
    pub thin_button_style: Style,
    pub giant_text_style: TextStyle,
    pub big_text_style: TextStyle,
    pub medium_text_style: TextStyle,
    pub small_text_style: TextStyle,
    pub tiny_text_style: TextStyle,
    pub tiny_red_text_style: TextStyle,
}

pub struct UiSpawnEventPlugin;

impl Plugin for UiSpawnEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTextsAndButtons>();
    }
}
