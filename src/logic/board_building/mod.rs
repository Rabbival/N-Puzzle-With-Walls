use crate::prelude::*;

pub mod board_builder;
pub mod solved_board_builder;
pub mod permutation_builder;
pub mod brute_force_builder;


pub struct BoardBuildingPlugins;

impl Plugin for BoardBuildingPlugins {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                SolvedBoardBuilderPlugin,
                BoardBuilderPlugin,
            ))
            ;
    }
}