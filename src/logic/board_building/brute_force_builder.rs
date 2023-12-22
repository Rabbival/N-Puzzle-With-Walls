use crate::{prelude::*, output::{print_to_console, error_handler}};

use rand::Rng;

/// a permutation that was made from shifts in a solved board 
/// would always be solvable (if we shift in reverse)
pub fn brute_force_generate_game_board(
    mut board: TileTypeBoard,
    generation_range: (u8, u8)
) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    let mut rng = rand::thread_rng();
    let mut location_shift_count=rng.gen_range(generation_range.0..generation_range.1);
    //prevents the generation of another solved board
    if location_shift_count%2 == 0 {
        location_shift_count+=1;
    }
    let mut empty_tile_location=board.empty_tile_location;

    let mut shift_direction_sequence:Vec<BasicDirection> = vec!();
    //we'll never shift with the location below on the first shift since there's none
    let mut previous_shift_direction = BasicDirection::Up; 
    for _shift in 0..location_shift_count{
        let mut optional_directions=
            board.get_all_direct_neighbor_locations(&empty_tile_location);

        //don't want to shift back and forth
        let opposite_of_previous_shift=previous_shift_direction.opposite_direction();
        if opposite_of_previous_shift.is_none(){
            return Err(error_handler::BoardGenerationError::DirectionCouldntBeFlipped);
        }
        optional_directions.remove(&opposite_of_previous_shift.unwrap());

        //choose, register, update board
        let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect(); 
        let chosen_shift_index=rng.gen_range(0..optional_directions.len());
        let chosen_direction=valid_directions[chosen_shift_index];
        let chosen_location_option=optional_directions.get(chosen_direction);
        if chosen_location_option.is_none(){
            return Err(error_handler::BoardGenerationError::ItemNotInMap
                (ItemNotFoundInMapError::DirectionNotFoundInMap));
        }
        let chosen_location=chosen_location_option.unwrap();
        if board.switch_tiles_by_location(&empty_tile_location, chosen_location).is_err(){
                return Err(error_handler::BoardGenerationError::TileMoveError);
            }
        
        //get ready for next choice
        empty_tile_location=board.empty_tile_location;
        shift_direction_sequence.push(*chosen_direction);
        previous_shift_direction= *chosen_direction;
    }

    //generation was successful
    let reveresed_shift_order=shift_direction_sequence
        .iter()
        .rev()
        .copied();
    print_to_console::print_possible_solution(reveresed_shift_order);
    Ok(board)
}


//board building tests
#[cfg(test)]
mod tests {
    use super::*;

    const RANDOM_RANGE_FOR_TESTING: (u8, u8) = (31,41);

    #[test]
    fn several_attempts_at_generating_unsolved_boards(){
        const ATTEMPT_COUNT: u8 = 10;
        let solved_board=generate_solved_board(DEFAULT_BOARD_SIDE_LENGTH);
        for _ in 0..ATTEMPT_COUNT{
            assert_ne!(solved_board.grid, 
                match generate_game_board(solved_board.clone(), RANDOM_RANGE_FOR_TESTING){
                    Ok(board)=> board,
                    Err(_)=> panic!()
                }.grid
            );
        }
    }
}