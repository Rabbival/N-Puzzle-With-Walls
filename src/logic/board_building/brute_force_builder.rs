use crate::{prelude::*, output::{print_to_console, error_handler}};

use rand::Rng;

pub fn brute_force_generate_game_board(
    solved_board: &TileBoard,
    generation_range: (u8, u8)
) -> Result<TileBoard, error_handler::BoardGenerationError>
{
    let mut rng = rand::thread_rng();
    let mut location_shift_count=rng.gen_range(generation_range.0..generation_range.1);
    // prevents the generation of another solved board
    if location_shift_count%2 == 0 {
        location_shift_count+=1;
    }
    let mut board = solved_board.clone();
    let empty_tile_locations = board.empty_tile_locations.clone();
    let mut location_shift_trackers : Vec<LocationShiftTracker>
        = empty_tile_locations.iter()
                                    .enumerate()
                                    .map(|(empty_index, empty_tile_location)|{
                                        LocationShiftTracker{
                                            empty_index,
                                            empty_location: *empty_tile_location,
                                            // we'll never shift with the location below 
                                            // on the first shift since there's none
                                            previous_shift: BasicDirection::Up,
                                            shift_direction_sequence: vec!()
                                        }
                                    })
                                    .collect();

    for _shift in 0..location_shift_count{
        for shift_tracker in &mut location_shift_trackers{
            let empty_tile_location = shift_tracker.empty_location;
            let mut optional_directions=
                board.get_direct_neighbor_locations_walls_excluded(&empty_tile_location);

            // don't want to shift back and forth, 
            // unless it's a dead end in which it has to turn back
            if optional_directions.len() > 1 {
                let opposite_of_previous_shift=shift_tracker.previous_shift.opposite_direction();
                if opposite_of_previous_shift.is_none(){
                    return Err(error_handler::BoardGenerationError::DirectionCouldntBeFlipped);
                }
                optional_directions.remove(&opposite_of_previous_shift.unwrap());
            }

            // choose, register, update board
            let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect(); 
            let chosen_shift_index=rng.gen_range(0..valid_directions.len());
            let chosen_direction=valid_directions[chosen_shift_index];
            let chosen_location_option=optional_directions.get(chosen_direction);
            if chosen_location_option.is_none(){
                return Err(error_handler::BoardGenerationError::ItemNotInMap
                    (ItemNotFoundInMapError::DirectionNotFoundInMap));
            }
            let chosen_location = chosen_location_option.unwrap();
            if let Err(error) = 
                board.swap_tiles_by_location(&empty_tile_location, chosen_location)
            {
                // if it tried to switch between two empty tiles
                // we'll ignore and not register the move
                match error{
                    TileMoveError::TriedToSwitchEmptyWithEmpty => continue,
                    _ => return Err(error_handler::BoardGenerationError::TileMoveError(error))
                };
            }else{
                //get ready for next iteration
                shift_tracker.empty_location = *chosen_location;
                shift_tracker.shift_direction_sequence.push(*chosen_direction);
                shift_tracker.previous_shift= *chosen_direction;
            }
        }
    }

    //generation was successful
    for shift_tracker in location_shift_trackers{
        let reveresed_shift_order=shift_tracker.shift_direction_sequence
            .iter()
            .rev()
            .copied();
        print_to_console::print_possible_solution(
            shift_tracker.empty_index,
            reveresed_shift_order
        );
    }
    
    Ok(board)
}


/// allows to keep track of multiple empty tiles
struct LocationShiftTracker{
    pub empty_index: usize,
    pub empty_location: GridLocation,
    pub previous_shift: BasicDirection,
    pub shift_direction_sequence: Vec<BasicDirection>
}



#[cfg(test)]
mod tests {
    use super::*;

    const RANDOM_RANGE_FOR_TESTING: (u8, u8) = (31,41);

    #[test]
    fn several_attempts_at_generating_unsolved_boards(){
        const ATTEMPT_COUNT: u8 = 10;
        let solved_board
            =generate_solved_board_inner(
                &BoardProperties::default(),
                &mut DataBaseManager::default()
            ).unwrap();
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
