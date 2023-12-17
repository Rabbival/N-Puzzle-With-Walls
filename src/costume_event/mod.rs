use crate::prelude::*;

pub mod reset_event;
pub mod move_tile_event;

pub struct EventPlugins;

impl Plugin for EventPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                ResetEventPlugin,
                MoveTileEventPlugin
            ))
            ;
    }
}