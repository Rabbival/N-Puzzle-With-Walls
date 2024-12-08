use crate::prelude::*;

pub mod app_event;
pub mod board_set_event;
pub mod db_event;
pub mod move_tile_event;
pub mod screen_changing_event;
pub mod shift_tiles_in_direction_request;
pub mod system_event;
pub mod ui_event;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ResetEventPlugin,
            MoveTileEventPlugin,
            ScreenChangingEventPlugin,
            UiEventPlugin,
            AppEventPlugin,
            DataBaseEventPlugin,
            SystemEventPlugin,
            ShiftTilesInDirectionRequestPlugin,
        ));
    }
}
