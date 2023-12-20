use crate::prelude::*;

pub mod enums;
pub mod board_manager;
pub mod data_structure;
pub mod tile_dictionary;
pub mod tile_type_board;
pub mod board_building;

pub struct BoardPlugins;

impl Plugin for BoardPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BoardBuildingPlugins,
                BoardManagerPlugin
            ))
            ;
    }
}