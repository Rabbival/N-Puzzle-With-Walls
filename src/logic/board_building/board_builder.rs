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
            =permutation_builder::generate_board_by_vector_permutation(solved_board.clone());
         //generation successful
        if let Ok(board) = attempt_result { 
            return Ok(board); 
        }
    }
    brute_force_builder::brute_force_generate_game_board(solved_board.clone(), generation_range)
}