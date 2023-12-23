use crate::{prelude::*, output::{print_to_console, error_handler}};

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
            .add_systems(Update, spawn_game_board)
            ;
    }
}


fn spawn_game_board(
    mut event_listener: EventReader<SpawnBoardWithNewSettings>,
    solved_board_query: Query<&TileTypeBoard, (With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileTypeBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    applied_board_prop_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
){
    for _event in event_listener.read(){
        let solved_board=solved_board_query.single();
        let board_size = applied_board_prop_query.single().size;
        let attempt_result=generate_game_board(
            solved_board.clone(),
            board_size.to_random_turns_range()
        );
        if let Ok(board) = attempt_result { 
            *game_board_query.single_mut() = board;
        }
        else{
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