use crate::prelude::*;

#[derive(Component)]
pub struct SolvedBoard;
#[derive(Component)]
pub struct GameBoard;

pub struct BoardBuilderPlugin;

impl Plugin for BoardBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::SolvedBoardGenerated),
                    build_a_new_board
            )
            .add_systems(
                OnEnter(GameState::PostGameBoardGenerationChangesDone),
                    declare_board_generation_done
                        .in_set(InputSystemSets::PostMainChanges)
            );
    }
}

fn build_a_new_board(
    mut generation_error_event_writer: EventWriter<ShowGenerationError>,
    solved_board_query: Query<&TileBoard, (With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let applied_props = applied_board_props_query.single();
    let solved_grid = &solved_board_query.single().grid;

    let optional_newborn_tiletype_board =
        TileBoard::try_from_solved_grid(solved_grid, applied_props.empty_count);
    match optional_newborn_tiletype_board {
        Err(error) => {
            generation_error_event_writer.send(ShowGenerationError(error));
            game_state.set(GameState::Regular);
        }
        Ok(newborn_board) => {
            let attempt_result =
                generate_game_board(
                    newborn_board, 
                    applied_props.get_random_turns_range()
                );
            match attempt_result {
                Ok(board) => {
                    game_state.set(GameState::GameBoardGenerated);
                    let mut game_board = game_board_query.single_mut();
                    *game_board = board;
                }
                Err(error) => {
                    game_state.set(GameState::Regular);
                    generation_error_event_writer.send(ShowGenerationError(error));
                }
            }
        }
    }
}

pub fn generate_game_board(
    solved_board: TileBoard,
    generation_range: (u8, u8),
) -> Result<TileBoard, BoardGenerationError> {
    // let attempt_result
    //     =generate_board_by_vector_permutation(&solved_board);
    //  //generation successful
    // if let Ok(board) = attempt_result {
    //     return Ok(board);
    // }

    brute_force_generate_game_board(&solved_board, generation_range)
}

fn declare_board_generation_done(
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    mut app_state: ResMut<NextState<AppState>>,
    current_app_state: Res<State<AppState>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    let mut game_board = game_board_query.single_mut();

    // if we're resetting when in game screen,
    // the board's input ignorance won't be toggled
    if let AppState::Game = current_app_state.get() {
        game_board.ignore_player_input = false;
    } else {
        app_state.set(AppState::Game);
    }
    
    game_log(GameLog::NewBoardGenerated);
    game_state.set(GameState::Regular);
}
