use crate::prelude::*;

pub mod board_building;
pub mod board_manager;
pub mod board_props;
pub mod data_structure;
pub mod enums;
pub mod states;
pub mod tile;
pub mod tile_dictionary;
pub mod tile_board;
pub mod ui_logic;

pub struct BoardPlugins;

impl Plugin for BoardPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BoardBuildingPlugins,
            BoardPropsPlugins,
            BoardManagerPlugin,
            UiLogicPlugin,
        ));
    }
}
