use crate::{prelude::*, output::{ error_handler, print_to_console}, costume_event::{board_set_event, ui_event}};

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
    mut generation_error_event_writer: EventWriter<ui_event::ShowGenerationError>,
    mut solved_board_query: Query<&mut TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_game_state: Res<State<GameState>>,
){
    for build_request in event_listener.read(){
        let mut solved_board_entity = solved_board_query.single_mut();
        let applied_props = applied_board_props_query.single();
        if build_request.reroll_solved {
            match generate_solved_board(applied_props){
                Ok(board) =>  *solved_board_entity = board,
                Err(error) => {
                    generation_error_event_writer
                        .send(ui_event::ShowGenerationError(error));
                    return;
                }
            }
        }
        let solved_grid = &solved_board_entity.grid;
        let mut game_board=game_board_query.single_mut();
        let optional_newborn_tiletype_board 
            = TileTypeBoard::from_grid(
                solved_grid, 
                applied_props.empty_count
            );
        match optional_newborn_tiletype_board{
            Err(error) => {
                generation_error_event_writer
                .send(ui_event::ShowGenerationError(error));
                    return;
            },
            Ok(newborn_board)=>{
                let attempt_result=
                    generate_game_board(
                        newborn_board, 
                        applied_props.size.to_random_turns_range()
                    );
                //generation successful
                match attempt_result{
                    Ok(board) =>  {
                        *game_board=board;
                        // if we're resetting when in game screen,
                        // the board's input ignorance won't be toggled
                        if let GameState::Game = current_game_state.get(){
                            game_board.ignore_player_input = false;
                        }else{
                            game_state.set(GameState::Game);
                        }
                        print_to_console::game_log(GameLog::NewBoardGenerated);
                    },
                    Err(error) => {
                        generation_error_event_writer
                            .send(ui_event::ShowGenerationError(error))
                    }
                }
            } 
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
            =permutation_builder::generate_board_by_vector_permutation(&solved_board);
         //generation successful
        if let Ok(board) = attempt_result { 
            return Ok(board); 
        }
    }

    brute_force_builder::brute_force_generate_game_board(&solved_board, generation_range)
}