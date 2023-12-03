use crate::prelude::*;
use rand::Rng;

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(Startup, generate_and_spawn_board)
            ;
    }
}

pub enum Tile {
    Empty,
    Numbered(u32),
}

fn generate_and_spawn_board(mut commands: Commands) {
    let mut board = Grid::<Tile>::default();
    let mut rng = rand::thread_rng();

    //make a solved board
    for i in 0..GRID_SIZE as u32 {
        for j in 0..GRID_SIZE as u32 {
            let location = GridLocation::new(i, j);
            board[&location] = Tile::Numbered(i*GRID_SIZE+j);
        }
    }
    board[&GridLocation::new(GRID_SIZE as u32, GRID_SIZE as u32)] = None;

    /*
    when spawning use:

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
