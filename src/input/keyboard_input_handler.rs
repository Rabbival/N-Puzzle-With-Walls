use crate::{prelude::*, logic::board_manager};

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
    mut tiles: Query<(&mut Transform, &Tile)>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::R){
        board_manager::reset_board(
            solved_board_query.single(),
            game_board_query.single_mut().into_inner(),
            tiles

        );
    }
}