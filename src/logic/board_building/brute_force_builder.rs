use rand::prelude::ThreadRng;
use crate::prelude::*;

use rand::Rng;

struct MoveDecidedAndRegistered(pub bool);

pub fn brute_force_generate_game_board(
    solved_board: &TileBoard,
    generation_range: (u8, u8)
) -> Result<TileBoard, BoardGenerationError>
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
            let move_registered = determine_next_shift_direction(
                &mut board,
                shift_tracker,
                &mut rng
            )?.0;
            if move_registered{
                
            }else{
                
            }
        }
    }

    //generation was successful
    for shift_tracker in location_shift_trackers{
        let reversed_shift_order=shift_tracker.shift_direction_sequence
            .iter()
            .rev()
            .copied();
        print_possible_solution(
            shift_tracker.empty_index,
            reversed_shift_order
        );
    }
    
    Ok(board)
}

fn determine_next_shift_direction(
    board: &mut TileBoard,
    shift_tracker: &mut LocationShiftTracker,
    rng: &mut ThreadRng
) -> Result<MoveDecidedAndRegistered, BoardGenerationError>
{
    let empty_tile_location = shift_tracker.empty_location;
    let mut optional_directions=
        board.get_direct_neighbor_locations_walls_excluded(&empty_tile_location);

    // don't want to shift back and forth,
    // unless it's a dead end in which it has to turn back
    if optional_directions.len() > 1 {
        remove_opposite_of_previous_shift_direction_from_possible_shift_locations(
            shift_tracker,
            &mut optional_directions
        )?;
    }

    let chosen_shift_direction = choose_next_shift_direction(
        &optional_directions,
        rng
    );
    let chosen_new_empty_tile_location = get_next_location_for_empty(
        &mut optional_directions,
        &chosen_shift_direction
    )?;

    let tile_swap_result =
        board.swap_tiles_by_location(&empty_tile_location, &chosen_new_empty_tile_location);
    match tile_swap_result {
        Err(error) => {
            match error{
                TileMoveError::TriedToSwitchEmptyWithEmpty => Ok(MoveDecidedAndRegistered(false)),
                _ => Err(BoardGenerationError::TileMoveError(error))
            }
        },
        Ok(_) => {
            prepare_shift_tracker_for_next_iteration(
                shift_tracker,
                chosen_new_empty_tile_location,
                chosen_shift_direction
            );
            Ok(MoveDecidedAndRegistered(true))
        }
    }
}

fn remove_opposite_of_previous_shift_direction_from_possible_shift_locations(
    shift_tracker: &LocationShiftTracker,
    optional_directions: &mut HashMap<BasicDirection, GridLocation>
) -> Result<(), BoardGenerationError>
{
    let opposite_of_previous_shift=shift_tracker.previous_shift.opposite_direction();
    match opposite_of_previous_shift {
        Some(opposite_of_prev) => {
            optional_directions.remove(&opposite_of_prev);
            Ok(())
        },
        None => {
            Err(BoardGenerationError::DirectionCouldntBeFlipped)
        }
    }
}

fn choose_next_shift_direction(
    optional_directions: &HashMap<BasicDirection, GridLocation>,
    rng: &mut ThreadRng
) -> BasicDirection
{
    let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect();
    let chosen_shift_index=rng.gen_range(0..valid_directions.len());
    *valid_directions[chosen_shift_index]
}

fn get_next_location_for_empty(
    optional_directions: &mut HashMap<BasicDirection, GridLocation>,
    chosen_direction: &BasicDirection
) -> Result<GridLocation, BoardGenerationError>
{
    let chosen_location_option=optional_directions.get(chosen_direction);
    match chosen_location_option{
        Some(chosen_location) => Ok(*chosen_location),
        None => {
            Err(BoardGenerationError::DirectionNotInMap
                (DataStructError::ItemNotFound(*chosen_direction)))
        }
    }
}

fn prepare_shift_tracker_for_next_iteration(
    shift_tracker: &mut LocationShiftTracker,
    updated_empty_tile_location: GridLocation,
    chosen_direction: BasicDirection
){
    shift_tracker.empty_location = updated_empty_tile_location;
    shift_tracker.shift_direction_sequence.push(chosen_direction);
    shift_tracker.previous_shift= chosen_direction;
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
    fn several_attempts_at_generating_unsolved_boards() {
        let mut app = App::new();
        app.add_systems(Update, several_attempts_at_generating_unsolved_boards_inner);
        app.update();
    }

    fn several_attempts_at_generating_unsolved_boards_inner(){
        const ATTEMPT_COUNT: u8 = 10;
        let mut tile_board = TileBoard::default();
        generate_solved_board_from_tile_board_with_walls_inner(
            &BoardProperties::default(),
            &mut tile_board
        ).unwrap();
        for _ in 0..ATTEMPT_COUNT{
            assert_ne!(tile_board.grid, 
                match generate_game_board(tile_board.clone(), RANDOM_RANGE_FOR_TESTING){
                    Ok(board)=> board,
                    Err(_)=> panic!()
                }.grid
            );
        }
    }
}
