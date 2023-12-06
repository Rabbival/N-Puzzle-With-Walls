use crate::{prelude::*, logic::board_manager, output::print_to_console};

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (listen_for_reset, move_tiles_with_keyboard));
    }
}

fn move_tiles_with_keyboard(keyboard_input: Res<Input<KeyCode>>){
    if keyboard_input.just_pressed(KeyCode::W) ||  keyboard_input.just_pressed(KeyCode::Up){
        info!("up just pressed");
    }
}

fn listen_for_reset(
    solved_board_query: Query<&Board,(With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut Board,(With<GameBoard>, Without<SolvedBoard>)>,
    tiles: Query<(Entity, &mut Tile, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::R){
        if let Err(error) = 
            board_manager::reset_board(
                solved_board_query.single(),
                &mut game_board_query.single_mut(),
                tiles
            )
        {
            print_to_console::print_debug_deriver(error, BevyPrintType::Error);
        }
    }
}