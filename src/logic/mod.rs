use crate::prelude::*;

pub mod states;
pub mod enums;
pub mod board_manager;
pub mod data_structure;
pub mod tile_dictionary;
pub mod tile_type_board;
pub mod board_building;
pub mod board_props;
pub mod ui_logic;

pub struct BoardPlugins;

impl Plugin for BoardPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BoardBuildingPlugins,
                BoardPropsPlugins,
                BoardManagerPlugin,
                UiLogicPlugin
            ))
            ;
    }
}