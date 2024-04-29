use crate::prelude::*;

pub mod board_builder;
pub mod board_entities_spawner;
pub mod brute_force_builder;
pub mod permutation_builder;
pub mod solved_board_builder;
pub mod wall_placement_validator;

pub struct BoardBuildingPlugin;

impl Plugin for BoardBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BoardEntitiesSpawnerPlugin, BoardBuilderPlugin, SolvedBoardPlugin));
    }
}
