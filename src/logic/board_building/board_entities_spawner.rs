use crate::prelude::*;

pub struct BoardEntitiesSpawnerPlugin;

impl Plugin for BoardEntitiesSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_deafult_boards);
    }
}

fn spawn_deafult_boards(mut commands: Commands) {
    commands.spawn((TileBoard::default(), SolvedBoard));
    commands.spawn((TileBoard::default(), GameBoard));
}
