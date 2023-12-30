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

#[derive (Event, Default)]
pub struct SpawnTileCountButtons{
    pub regular_button_style: Style,
    pub thin_button_style: Style,
    pub button_text_style: TextStyle
}

#[derive (Event)]
pub struct ButtonPressed{
    pub entity: Entity,
    pub action: MenuButtonAction
}

#[derive (Event)]
pub struct ApplyButtonPressed{
    pub action: MenuButtonAction
}


pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnButtons>()
            .add_event::<SpawnBigButtons>()
            .add_event::<SpawnTileCountButtons>()
            .add_event::<ButtonPressed>()
            .add_event::<ApplyButtonPressed>()
            ;
    }
}