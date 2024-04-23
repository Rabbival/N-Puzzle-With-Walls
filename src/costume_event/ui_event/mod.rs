use crate::prelude::*;

pub mod ui_spawn_event;
pub mod menu_ui_event;
pub mod loader_ui_event;
pub mod game_ui_event;


#[derive(Event)]
pub struct ResetButtonTextColor;

#[derive(Event)]
pub struct ToggleButton {
    pub entity: Entity,
}

#[derive(Event)]
pub struct SetEntityVisibility {
    pub entity: Entity,
    pub visibility: Visibility
}

pub struct UiEventPlugin;

impl Plugin for UiEventPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                UiSpawnEventPlugin,
                MenuUiEventPlugin,
                LoaderUiEventPlugin,
                GameUiEventPlugin,
            ))
            .add_event::<ResetButtonTextColor>()
            .add_event::<ToggleButton>()
            .add_event::<SetEntityVisibility>();
    }
}