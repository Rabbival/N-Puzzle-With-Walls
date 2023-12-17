use bevy::prelude::*;
use crate::prelude::*;

pub mod basic_direction;
pub mod tile_type;
pub mod board_manager;
pub mod grid;
pub mod tile_dictionary;
pub mod tile_type_board;
pub mod board_builder;


pub struct BoardPlugins;

impl Plugin for BoardPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                BoardBuilderPlugin,
                BoardManagerPlugin
            ))
            ;
    }
}