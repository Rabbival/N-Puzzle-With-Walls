use crate::prelude::*;

pub mod board_properties;
pub mod update_board_properties;
pub mod current_board_wall_locations;
pub mod unapllied_menu_wall_count;

pub struct BoardPropsPlugin;

impl Plugin for BoardPropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BoardPropertiesPlugin, 
            UpdateBoardPropertiesPlugin,
            CurrentBoardWallLocationsPlugin,
            UnappliedMenuWallCountPlugin
        ));
    }
}
