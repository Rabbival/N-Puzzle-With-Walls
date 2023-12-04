use crate::{prelude::*, output::error_handler};
use rand::Rng;

const LOCATION_SHIFT_BOUNDS:(u8, u8) = (18, 32);
const BOARD_GENERATION_ATTEMPTS:u8=5;

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            //important to run before we draw it in graphics.rs
            .add_systems(PreStartup, generate_board_or_panic)
            ;
    }
}

fn generate_board_or_panic(mut commands: Commands){
    for attempt in 0..BOARD_GENERATION_ATTEMPTS{
        let attempt_result=generate_board(&mut commands);
        if let Ok(()) = attempt_result { return; } //generation successful
    }
    panic!("couldn't generate board");
}

/// a permutation that was made from shifts in a solved board 
/// would always be solvable (if we shift in reverse)
fn generate_board(commands: &mut Commands) -> Result<(), error_handler::DirectionRelatedError>{
    let (mut empty_tile_location, mut board)=initialize_to_solved();
    let mut rng = rand::thread_rng();
    let location_shift_count=rng.gen_range(LOCATION_SHIFT_BOUNDS.0..LOCATION_SHIFT_BOUNDS.1);

    let mut shift_direction_sequence:Vec<BasicDirection> = vec!();
    //we'll never shift with the location below on the first shift since there's none
    let mut previous_shift_direction = BasicDirection::Up; 
    for shift in 1..location_shift_count{
        let mut optional_directions=
            board.get_all_direct_neighbor_locations(&empty_tile_location);
        //don't want to shift back and forth
        if let None = previous_shift_direction.opposite_direction(){
            return Err(error_handler::DirectionRelatedError::DirectionCouldntBeFlipped);
        }
        let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect(); 
        let chosen_shift_index=rng.gen_range(0..optional_directions.len());
        let chosen_direction=valid_directions[chosen_shift_index];
        let chosen_location_option=optional_directions.get(chosen_direction);
        if let None = chosen_location_option{
            return Err(error_handler::DirectionRelatedError::DirectionNotFoundInMap);
        }
        let chosen_location=chosen_location_option.unwrap();
        board.switch_tiles_by_location(&empty_tile_location, chosen_location);
        empty_tile_location=chosen_location.clone();
        shift_direction_sequence.push(*chosen_direction);
        previous_shift_direction=*chosen_direction;
    }

    commands.insert_resource(board);
    Ok(())
}

fn initialize_to_solved() -> (GridLocation, Board){
    let mut solved_board = Board::default();
    for i in 0..GRID_SIZE as u32 {
        for j in 0..GRID_SIZE as u32 {
            let location = GridLocation::new(i as i32, j as i32);
            solved_board[&location] = Tile::new(Some(i*GRID_SIZE+j));
        }
    }
    let empty_tile_location=GridLocation::new(GRID_SIZE as i32, GRID_SIZE as i32);
    solved_board[&empty_tile_location] = Tile::new(None);
    (empty_tile_location, solved_board)
}

/// check if there's an empty space next to it (in straight line)
/// if there is:
///  - move its entity (using graphics)
///  - move its logic tile (using grid)
pub fn move_tile_logic(
    location: GridLocation, 
    board: Res<Board>
) -> Result<(), error_handler::InputHandlerError>
{
    if !board.occupied(&location) {
        return Ok(());
    }
    
    //if there's no empty neighbor throw error_handler::GridLocationOccupied

    return Ok(());
}