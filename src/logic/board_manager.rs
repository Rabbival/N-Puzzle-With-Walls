use crate::{prelude::*, output::{error_handler, print_to_console}};
use rand::Rng;

const LOCATION_SHIFT_BOUNDS:(u8, u8) = (8, 22);
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
    let (empty_tile_location, solved_board)=initialize_to_solved();
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        let attempt_result=generate_board(
            &mut commands,
            (empty_tile_location.clone(), solved_board.clone())
        );
        if let Ok(()) = attempt_result { return; } //generation successful
    }
    print_to_console::couldnt_generate_board();
}

/// a permutation that was made from shifts in a solved board 
/// would always be solvable (if we shift in reverse)
fn generate_board(
    commands: &mut Commands,
    (mut empty_tile_location, mut board): (GridLocation, Board)
) -> Result<(), error_handler::BoardGenerationError>
{
    let mut rng = rand::thread_rng();
    let mut location_shift_count=rng.gen_range(LOCATION_SHIFT_BOUNDS.0..LOCATION_SHIFT_BOUNDS.1);
    //preventing the generation of a solved board
    if location_shift_count%2 == 0 {
        location_shift_count+=1;
    }

    let mut shift_direction_sequence:Vec<BasicDirection> = vec!();
    //we'll never shift with the location below on the first shift since there's none
    let mut previous_shift_direction = BasicDirection::Up; 
    for _shift in 0..location_shift_count{
        let mut optional_directions=
            board.get_all_direct_neighbor_locations(&empty_tile_location);

        //don't want to shift back and forth
        let opposite_of_previous_shift=previous_shift_direction.opposite_direction();
        if let None = opposite_of_previous_shift{
            return Err(error_handler::BoardGenerationError::DirectionCouldntBeFlipped);
        }
        optional_directions.remove(&opposite_of_previous_shift.unwrap());

        //choose, register, update board
        let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect(); 
        let chosen_shift_index=rng.gen_range(0..optional_directions.len());
        let chosen_direction=valid_directions[chosen_shift_index];
        let chosen_location_option=optional_directions.get(chosen_direction);
        if let None = chosen_location_option{
            return Err(error_handler::BoardGenerationError::DirectionNotFoundInMap);
        }
        let chosen_location=chosen_location_option.unwrap();
        board.switch_tiles_by_location(&empty_tile_location, chosen_location);
        
        //get ready for next choice
        empty_tile_location=chosen_location.clone();
        shift_direction_sequence.push(*chosen_direction);
        previous_shift_direction=*chosen_direction;
    }

    //generation was successful
    let reveresed_shift_order=shift_direction_sequence
        .iter()
        .map(|direction| -> BasicDirection {
            direction.opposite_direction().unwrap()
        });
    print_to_console::print_possible_solution(reveresed_shift_order);
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
    let empty_tile_location=GridLocation::new((GRID_SIZE-1) as i32, (GRID_SIZE-1) as i32);
    solved_board[&empty_tile_location] = Tile::new(None);
    (empty_tile_location, solved_board)
}

/// check if there's an empty space next to it (in straight line)
/// if there is:
///  - move its entity (using graphics)
///  - move its logic tile (using grid)
pub fn move_tile_logic(
    location: GridLocation, 
    board: ResMut<Board>
) -> Result<(), error_handler::InputHandlerError>
{
    if !board.occupied(&location) {
        return Err(InputHandlerError::PressedEmptySlot(String::from("pressed an empty slot")));
    }
    let optional_empty_neighbor= board.clone().get_empty_neighbor(&location);
    if let None=optional_empty_neighbor{
        return Err(InputHandlerError::NoEmptyNeighbor(String::from("no empty neighbor")));
    }
    let empty_neighbor=optional_empty_neighbor.unwrap();

    board.clone().switch_tiles_by_location(&empty_neighbor, &location);

    //TODO: move entities

    return Ok(());
}