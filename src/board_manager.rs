use crate::prelude::*;
use rand::Rng;

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(Startup, (generate_board, spawn_board))
            ;
    }
}

pub enum TileType {
    Empty,
    Numbered(u32),
}

fn generate_board() {
    let mut board = Grid::<TileType>::default();
    let mut rng = rand::thread_rng();

    //make a solved board
    for i in 0..GRID_SIZE as u32 {
        for j in 0..GRID_SIZE as u32 {
            let location = GridLocation::new(i, j);
            board[&location] = TileType::Numbered(i*GRID_SIZE+j);
        }
    }
    board[&GridLocation::new(GRID_SIZE as u32, GRID_SIZE as u32)] = TileType::None;

    //generate a list of directions (that can be reversed to solve)

    
}

fn spawn_board(mut commands: Commands){
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
