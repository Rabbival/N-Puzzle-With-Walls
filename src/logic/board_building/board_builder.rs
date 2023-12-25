use crate::{prelude::*, output::{print_to_console, error_handler}, costume_event::board_set_event};

use super::{permutation_builder, brute_force_builder};

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
            .add_systems(Update, 
                build_a_new_board.in_set(InputSystemSets::ChangesBasedOnInput)
            )
            ;
    }
}


fn build_a_new_board(
    mut event_listener: EventReader<board_set_event::BuildNewBoard>,
    mut solved_board_query: Query<&mut TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
    applied_board_prop_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
){
    for build_request in event_listener.read(){
        let mut solved_board_entity = solved_board_query.single_mut();
        let board_size = applied_board_prop_query.single().size;
        if build_request.reroll_solved {
            *solved_board_entity = generate_solved_board(board_size.to_grid_side_length());
        }
        let solved_grid = &solved_board_entity.grid;
        let mut game_board=game_board_query.single_mut();
        let attempt_result=
            generate_game_board(
                TileTypeBoard::from_grid(solved_grid), 
                board_size.to_random_turns_range()
            );
        //generation successful
        if let Ok(board) = attempt_result { 
            *game_board=board;
            
        }else{
            print_to_console::couldnt_generate_board();
        }
    }
}

pub fn generate_game_board(
    solved_board: TileTypeBoard,
    generation_range: (u8, u8)
) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        let attempt_result
            =permutation_builder::generate_board_by_vector_permutation(solved_board.clone());
         //generation successful
        if let Ok(board) = attempt_result { 
            return Ok(board); 
        }
    }
    brute_force_builder::brute_force_generate_game_board(solved_board.clone(), generation_range)
}