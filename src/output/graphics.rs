use crate::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_board)
            ;
    }
}

fn spawn_board(mut commands: Commands){
    /*
    draft:

    commands.spawn((
        SpatialBundle::default(),
        Target {
            use_offset: IVec2 { x: 0, y: -1 },
        },
        LockToGrid,
        GridLocation::new(10, 10),
        //Tile::     ///depending on what's in there
    ));
         */
}

pub fn move_tile_to_empty(tile: Entity, from: &GridLocation, to: &GridLocation){

}