use crate::prelude::*;
use rand::Rng;

const LOCATION_SHIFT_BOUNDS:(u8, u8) = (18, 32);

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(Startup, (generate_board, spawn_board))
            ;
    }
}

#[derive(Component, Clone, Copy)]
pub enum TileType {
    Empty,
    Numbered(u32),
}

fn generate_board() {
    let mut board = Grid::default();
    let mut rng = rand::thread_rng();

    //make a solved board
    for i in 0..GRID_SIZE as u32 {
        for j in 0..GRID_SIZE as u32 {
            let location = GridLocation::new(i as i32, j as i32);
            board[&location] = TileType::Numbered(i*GRID_SIZE+j);
        }
    }
    let empty_tile_location=&GridLocation::new(GRID_SIZE as i32, GRID_SIZE as i32);
    board[empty_tile_location] = TileType::Empty;

    //a permutation that was made from shifts in a solved board would always be solvable (if we shift in reverse)
    let mut rng = rand::thread_rng();
    let location_shift_count=rng.gen_range(LOCATION_SHIFT_BOUNDS.0..LOCATION_SHIFT_BOUNDS.1);
    let shift_direction_sequence:Vec<BasicDirection> = vec!();
    //we'll never shift with the location below on the first shift since there's none
    let previous_shift_direction = BasicDirection::Up; 
    for shift in 1..location_shift_count{
        let mut optional_directions=board.get_all_direct_neighbors(empty_tile_location);
        //don't want to shift back and forth
        optional_directions.remove(&previous_shift_direction.opposite_direction());
        let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect(); 
        let chosen_shift_index=rng.gen_range(0..optional_directions.len());
        let chosen_direction=valid_directions[chosen_shift_index];
        let chosen_location=optional_directions.get(chosen_direction);

        /*
            - switch between the tiles in empty_tile_location and chosen_location
            - empty_tile_location=chosen_location
            - push chosen_direction into shift_direction_sequence
            - previous_shift_direction=chosen_direction
         */
    }

    
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
