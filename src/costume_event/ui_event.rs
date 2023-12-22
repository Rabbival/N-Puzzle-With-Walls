use crate::prelude::*;

#[derive (Event, Default)]
pub struct SpawnButtons{
    pub button_style: Style,
    pub button_text_style: TextStyle
}

#[derive (Event, Default)]
pub struct SpawnBigButtons{
    pub big_button_style: Style,
    pub big_button_text_style: TextStyle
}


pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnButtons>()
            .add_event::<SpawnBigButtons>()
            ;
    }
}