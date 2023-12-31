use crate::prelude::*;

pub mod board_set_event;
pub mod move_tile_event;
pub mod screen_changing_event;
pub mod ui_event;
pub mod ui_spawn_event;
pub mod app_event;

pub struct EventPlugins;

impl Plugin for EventPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ResetEventPlugin,
                MoveTileEventPlugin,
                ScreenChangingEventPlugin,
                UiEventPlugin,
                UiSpawnEventPlugin,
                AppEventPlugin,
            ))
            ;
    }
}