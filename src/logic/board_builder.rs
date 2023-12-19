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
    mut board: TileTypeBoard
) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    let sorted_vector = make_sorted_types_vector(board.get_side_length());
    let permutation 
        = make_valid_permutation_out_of_vector(&sorted_vector);

    //TODO: put permutation in board and return it

}

fn make_sorted_types_vector(length: &u8) -> Vec<TileType> {
    let mut output_vector=vec![];
    for index in 0..length-1{
        output_vector.push(TileType::Numbered(index as u32));
    }
    output_vector.push(TileType::Empty);
    output_vector
}

fn make_valid_permutation_out_of_vector(sorted_vector: &Vec<TileType>) 
-> Result<Vec<TileType>, error_handler::BoardGenerationError> 
{
    let mut permutation_result
        = Err(error_handler::BoardGenerationError::VectorPermutationGenerationFailed);
    let mut rng = rand::thread_rng();
    let mut permutation = vec![];
    let permutation_length = sorted_vector.len();
    
    for attempt in 0..BOARD_GENERATION_ATTEMPTS{
        //generate random permutation
        let mut cloned_sorted = sorted_vector.clone();
        let mut cloned_sorted_size = permutation_length;
        permutation = vec![];
        for index in 0..sorted_vector.len(){
            let chosen_index=rng.gen_range(0..cloned_sorted_size);
            permutation.push(cloned_sorted.remove(chosen_index));
            cloned_sorted_size -= 1;
        }
        if validate_solvability(sorted_vector, &permutation){
            permutation_result=Ok(permutation);
            break;
        }
    }

    permutation_result
}

fn validate_solvability(sorted_vector: &Vec<TileType>, permutation: &Vec<TileType>) -> bool {
    let mut wrong_place_counter = 0;
    for (sorted_value, permutation_value) in sorted_vector.iter().zip(permutation.iter()) {
        if sorted_value != permutation_value{
            wrong_place_counter += 1;
        }
    }
    wrong_place_counter % 2 == 0
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