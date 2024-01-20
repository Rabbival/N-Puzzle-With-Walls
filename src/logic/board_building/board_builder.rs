use crate::{
    costume_event::{board_set_event, ui_event},
    output::{error_handler, print_to_console},
    prelude::*,
};

use super::{brute_force_builder, permutation_builder};

pub const BOARD_GENERATION_ATTEMPTS: u8 = 5;

#[derive(Component)]
pub struct SolvedBoard;
#[derive(Component)]
pub struct GameBoard;

pub struct BoardBuilderPlugin;

impl Plugin for BoardBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            //important to run before we draw it in graphics.rs
            .add_systems(
                Update,
                build_a_new_board.in_set(InputSystemSets::PostInitialChanges),
            );
    }
}

fn build_a_new_board(
    mut event_listener: EventReader<board_set_event::BuildNewBoard>,
    mut generation_error_event_writer: EventWriter<ui_event::ShowGenerationError>,
    mut camera_adjustmant_event_writer: EventWriter<SetCameraAccordingToNewSettings>,
    solved_board_query: Query<&TileBoard, (With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<AppState>>,
    current_game_state: Res<State<AppState>>,
) {
    for _build_request in event_listener.read() {
        let applied_props = applied_board_props_query.single();
        let solved_grid = &solved_board_query.single().grid;
        let mut game_board = game_board_query.single_mut();
        let optional_newborn_tiletype_board =
            TileBoard::from_grid(solved_grid, applied_props.empty_count);
        match optional_newborn_tiletype_board {
            Err(error) => {
                generation_error_event_writer.send(ui_event::ShowGenerationError(error));
                return;
            }
            Ok(newborn_board) => {
                let attempt_result =
                    generate_game_board(newborn_board, applied_props.size.to_random_turns_range());
                //generation successful
                match attempt_result {
                    Ok(board) => {
                        *game_board = board;
                        // if we're resetting when in game screen,
                        // the board's input ignorance won't be toggled
                        if let AppState::Game = current_game_state.get() {
                            game_board.ignore_player_input = false;
                        } else {
                            game_state.set(AppState::Game);
                        }
                        print_to_console::game_log(GameLog::NewBoardGenerated);
                        camera_adjustmant_event_writer.send(board_set_event::SetCameraAccordingToNewSettings {
                            new_grid_side_length: applied_props.size.to_grid_side_length(),
                        });
                    }
                    Err(error) => {
                        generation_error_event_writer.send(ui_event::ShowGenerationError(error))
                    }
                }
            }
        }
    }
}

pub fn generate_game_board(
    solved_board: TileBoard,
    generation_range: (u8, u8),
) -> Result<TileBoard, error_handler::BoardGenerationError> {
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
