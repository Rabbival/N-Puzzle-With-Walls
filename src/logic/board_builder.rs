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
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        let attempt_result=generate_game_board(
            solved_board.clone(),
            board_size_res.to_random_turns_range()
        );
         //generation successful
        if let Ok(board) = attempt_result { 
            commands.spawn((
                board,
                GameBoard
            ));
            return; 
        }
    }
    print_to_console::couldnt_generate_board();
}

/// a permutation that was made from shifts in a solved board 
/// would always be solvable (if we shift in reverse)
pub fn generate_game_board(
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
            return Err(error_handler::BoardGenerationError::ItemNotInMap
                (ItemNotFoundInMapError::DirectionNotFoundInMap));
        }
        let chosen_location=chosen_location_option.unwrap();
        if let Err(_) = 
            board.switch_tiles_by_location(&empty_tile_location, chosen_location){
                return Err(error_handler::BoardGenerationError::TileMoveError);
            }
        
        //get ready for next choice
        empty_tile_location=board.empty_tile_location;
        shift_direction_sequence.push(*chosen_direction);
        previous_shift_direction=*chosen_direction;
    }

    //generation was successful
    let reveresed_shift_order=shift_direction_sequence
        .iter()
        .rev()
        .map(|direction| -> BasicDirection {
            *direction
        });
    print_to_console::print_possible_solution(reveresed_shift_order);
    board.ignore_player_input=false;
    Ok(board)
}