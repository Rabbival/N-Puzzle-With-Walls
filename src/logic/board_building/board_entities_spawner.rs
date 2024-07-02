use crate::prelude::*;

pub struct BoardEntitiesSpawnerPlugin;

impl Plugin for BoardEntitiesSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_default_boards);
    }
}

fn spawn_default_boards(mut commands: Commands) {
    commands.spawn((TileBoard::default(), SolvedBoard));
    commands.spawn((TileBoard::default(), GameBoard, DomainBoardName::default()));
}
