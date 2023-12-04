use crate::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_board)
            ;
    }
}

fn spawn_board(
    mut commands: Commands,
    board: Res<Board>
){
    for row in board.grid{
        for tile_from_cell in row{
            commands.spawn((
                SpatialBundle::default(),
                GridLocation::new(10, 10),
                tile_from_cell,
                
                //spirte?
            ));
        }
    }

}

pub fn move_tile_entity(tile: Entity, from: &GridLocation, to: &GridLocation){

}