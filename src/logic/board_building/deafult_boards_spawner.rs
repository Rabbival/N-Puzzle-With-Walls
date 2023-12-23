use crate::prelude::*;


pub struct DefaultBoardsSpawnerPlugin;

impl Plugin for DefaultBoardsSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, 
                spawn_deafult_boards
            )
            ;
    }
}


fn spawn_deafult_boards(
    mut commands: Commands,
){
    commands.spawn((TileTypeBoard::default(), SolvedBoard));
    commands.spawn((TileTypeBoard::default(), GameBoard));
}