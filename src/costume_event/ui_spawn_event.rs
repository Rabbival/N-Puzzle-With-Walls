use crate::prelude::*;

#[derive(Event, Default)]
pub struct SpawnButtons {
    pub button_style: Style,
    pub button_text_style: TextStyle,
}

#[derive(Event, Default)]
pub struct SpawnBigButtons {
    pub big_button_style: Style,
    pub big_button_text_style: TextStyle,
}

#[derive(Event, Default)]
pub struct SpawnTileCountButtons {
    pub regular_button_style: Style,
    pub thin_button_style: Style,
    pub button_text_style: TextStyle,
    pub small_text_style: TextStyle,
}

#[derive(Event, Default)]
pub struct SpawnEternalButtons {
    pub thin_button_style: Style,
    pub button_text_style: TextStyle,
}

pub struct UiSpawnEventPlugin;

impl Plugin for UiSpawnEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnButtons>()
            .add_event::<SpawnBigButtons>()
            .add_event::<SpawnTileCountButtons>()
            .add_event::<SpawnEternalButtons>();
    }
}
