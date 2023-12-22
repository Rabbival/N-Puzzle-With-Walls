use crate::prelude::*;

pub mod reset_event;
pub mod move_tile_event;
pub mod screen_unloading_event;
pub mod ui_event;

pub struct EventPlugins;

impl Plugin for EventPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ResetEventPlugin,
                MoveTileEventPlugin,
                ScreenUnloadingEventPlugin,
                UiEventPlugin
            ))
            ;
    }
}