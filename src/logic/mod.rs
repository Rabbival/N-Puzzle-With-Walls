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
pub mod loader_screen_logic;
pub mod multiple_empty_tiles_choice_manager;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BoardBuildingPlugin,
            BoardPropsPlugin,
            BoardManagerPlugin,
            UiLogicPlugin,
            LoaderScreenLogicPlugin,
            MultipleEmptyTilesChoiceManagerPlugin,
        ));
    }
}
