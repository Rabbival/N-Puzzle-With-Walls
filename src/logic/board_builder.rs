use crate::{prelude::*, output::{print_to_console, error_handler}};
use rand::Rng;

pub const BOARD_GENERATION_ATTEMPTS:u8=5;

#[derive(Component)]
pub struct SolvedBoard;
#[derive(Component)]
pub struct GameBoard;

pub struct BoardBuilderPlugin;

impl Plugin for BoardBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            //important to run before we draw it in graphics.rs
            .add_systems(Startup, spawn_game_board)
            ;
    }
}


fn spawn_game_board(
    mut commands: Commands, 
    query: Query<&TileTypeBoard, With<SolvedBoard>>,
    board_size_res: Res<BoardSize>
){
    let solved_board=query.single();
    let attempt_result=generate_game_board(
        solved_board.clone(),
        board_size_res.to_random_turns_range()
    );
    if let Ok(board) = attempt_result { 
        commands.spawn((
            board,
            GameBoard
        ));
    }
    else{
        print_to_console::couldnt_generate_board();
    }
}

pub fn generate_game_board(
    solved_board: TileTypeBoard,
    generation_range: (u8, u8)
) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        let attempt_result
            =generate_board_by_vector_permutation(solved_board.clone());
         //generation successful
        if let Ok(board) = attempt_result { 
            return Ok(board); 
        }
    }
    brute_force_generate_game_board(solved_board.clone(), generation_range)
}

fn generate_board_by_vector_permutation(
    board: TileTypeBoard
) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    let solved_board_iterator = board.grid.iter();
    let mut sorted_tiles=vec![];
    let mut sorted_indexes=vec![];
    for (index, optional_tile) in solved_board_iterator{
        sorted_indexes.push(index);
        if optional_tile.is_none(){
            return Err(error_handler::BoardGenerationError::GridError(GridError::IteratorYieldedNone));
        }else{
            sorted_tiles.push(*optional_tile.unwrap());
        }
    }
    let permutation 
        = make_valid_permutation_out_of_vector(&sorted_tiles)?;
    let mut grid=Grid::new(*board.get_side_length());
    for (location, value) in sorted_indexes.iter().zip(permutation.iter()){
        grid.set(location, *value);
    }
    Ok(TileTypeBoard::from_grid(&grid))
}

fn make_valid_permutation_out_of_vector(sorted_vector: &Vec<TileType>) 
-> Result<Vec<TileType>, error_handler::BoardGenerationError> 
{
    let mut permutation_result
        = Err(error_handler::BoardGenerationError::VectorPermutationGenerationFailed);
    let mut rng = rand::thread_rng();
    let mut permutation ;
    let permutation_length = sorted_vector.len();
    
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        //generate random permutation
        let mut cloned_sorted = sorted_vector.clone();
        let mut cloned_sorted_size = permutation_length;
        permutation = vec![];
        for _index in 0..sorted_vector.len(){
            let chosen_index=rng.gen_range(0..cloned_sorted_size);
            permutation.push(cloned_sorted.swap_remove(chosen_index));
            cloned_sorted_size -= 1;
        }
        if validate_and_attempt_solvability(sorted_vector, &mut permutation){
            permutation_result=Ok(permutation);
            break;
        }
    }

    permutation_result
}

fn validate_and_attempt_solvability(sorted_vector: &Vec<TileType>, permutation: &mut Vec<TileType>) -> bool {
    let mut wrong_placed = vec![];
    for ((item_index, sorted_value), permutation_value) 
        in sorted_vector.iter().enumerate().zip(permutation.iter()) 
    {
        if sorted_value != permutation_value{
            wrong_placed.push(
                IndexedValue::<usize>{
                    index: item_index,
                    value: permutation_value.to_number_forced(permutation.len())
                }
            );
        }
    }
    if wrong_placed.len() % 2 == 0 {
        true
    }else{
        attempt_solvability(&mut wrong_placed, permutation)
    }
}

/// tries to replace a wrong-placed with another wrong-placed that's shouldn't be in its place
/// in order to make their amount even
/// returns whether the attempt was successful
fn attempt_solvability(wrong_placed: &mut Vec<IndexedValue::<usize>>, permutation: &mut Vec<TileType>)
-> bool
{
    while let Some(wrong_placed_element) = wrong_placed.pop(){
        let numeric_value = wrong_placed_element.value;
        //in a solved board, numbers start at 1 at index 0
        let correct_location=numeric_value-1;
        //if it wasn't found, the corresponding number is correct 
        //and thus switching would keep an odd number of wrong-placed
        if let Ok(index_in_wrongs) = wrong_placed
            .binary_search_by_key(
                &correct_location, 
                |&indexed_value| indexed_value.index
            )
        {
            let elememt_in_correct_location 
                = wrong_placed.get(index_in_wrongs);
            //we can unwrap directly since it was returned as Ok()
            let number_in_correct_location = elememt_in_correct_location.unwrap().value;
            //putting two wrong numbers in the right place keeps the number of wrongs odd
            if number_in_correct_location != (wrong_placed_element.index - 1) {
                permutation.swap(
                    wrong_placed_element.index, 
                    elememt_in_correct_location.unwrap().index
                );
                break;
            }
        }
    }
    false
}


/// a permutation that was made from shifts in a solved board 
/// would always be solvable (if we shift in reverse)
fn brute_force_generate_game_board(
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
    board.ignore_player_input=false;
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

    

    //TODO: test:
    //attempt_solvability 
    /*
    examples:
    only one in wrong place (should be false)
    three in wrong place (should be true)
     */
    //validate_and_attempt_solvability
    /*
    examples:
    sorted (should be true)
    two wrong places (should be true)
    only one in wrong place (should be false)
     */
    //make_valid_permutation_out_of_vector
    /*
    I should check that the returned one has an even number of tiles in wrong places
     */
}