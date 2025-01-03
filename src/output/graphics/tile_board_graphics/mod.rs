use crate::output::graphics::tile_board_graphics::in_game_tile_graphics::InGameTileGraphicsPlugin;
use crate::prelude::*;

pub mod board_spawning_tile_graphics;
pub mod in_game_tile_graphics;
pub mod tile_addons_spawner;
pub mod in_game_arrows_graphics;

pub struct TileGraphicsPlugin;

impl Plugin for TileGraphicsPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
                BoardSpawningTileGraphicsPlugin,
                TileAddonsSpawnerPlugin,
                InGameTileGraphicsPlugin,
                InGameArrowsGraphicsPlugin
            )
        );
    }
}