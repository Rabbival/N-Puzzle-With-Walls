use crate::{prelude::*, logic::basic_direction, output::{print_to_console, error_handler}, costume_event::{reset_event, move_tile_event}};

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    move_tiles_with_keyboard,
                    listen_for_reset, 
                )
                .chain()
                .in_set(CostumeSystemSets::InputListening)
            )
            ;
    }
}

fn move_tiles_with_keyboard(
    mut logic_event_writer: EventWriter<move_tile_event::SwitchTilesLogic>,
    game_board_query: Query<&TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
    keyboard_input: Res<Input<KeyCode>>,
){
    let mut move_request_direction:Option<basic_direction::BasicDirection>=None;
    if keyboard_input.just_pressed(KeyCode::W) ||  keyboard_input.just_pressed(KeyCode::Up){
        move_request_direction=Some(basic_direction::BasicDirection::Down);
    }
    if keyboard_input.just_pressed(KeyCode::D) ||  keyboard_input.just_pressed(KeyCode::Right){
        move_request_direction=Some(basic_direction::BasicDirection::Left);
    }
    if keyboard_input.just_pressed(KeyCode::S) ||  keyboard_input.just_pressed(KeyCode::Down){
        move_request_direction=Some(basic_direction::BasicDirection::Up);
    }
    if keyboard_input.just_pressed(KeyCode::A) ||  keyboard_input.just_pressed(KeyCode::Left){
        move_request_direction=Some(basic_direction::BasicDirection::Right);
    }
    if let None = move_request_direction {
        return;
    }
    if let Err(error) = move_into_empty_from_direction(
        &mut logic_event_writer,
        game_board_query.single(),
        move_request_direction.unwrap(),
    ){
        print_to_console::print_tile_move_error(error)
    }
}

fn move_into_empty_from_direction(
    logic_event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    game_board: &TileTypeBoard,
    move_to_direction: basic_direction::BasicDirection,
) -> Result<(), error_handler::TileMoveError>
{
    if game_board.ignore_player_input{
        return Err(error_handler::TileMoveError::BoardFrozenToPlayer(String::from("board locked")));
    }
    let empty_tile_neighbors=game_board.get_direct_neighbors_of_empty();
    if let Some(occupied_tile_location) = empty_tile_neighbors.get(&move_to_direction){
        logic_event_writer.send(move_tile_event::SwitchTilesLogic{
            occupied_tile_location: *occupied_tile_location, 
            empty_tile_location: game_board.empty_tile_location,
        });
        Ok(()) //only here for the sake of testing, there will always be tiles.
    }else{
        Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(move_to_direction))
    }
}

fn listen_for_reset(
    mut input_event_writer: EventWriter<reset_event::ResetBoardLogic>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::R){
        input_event_writer.send(reset_event::ResetBoardLogic::default());
    }
}


#[cfg(test)]
mod tests {
    use crate::logic::solved_board_builder;

    use super::*;

    #[test]
    fn test_valid_request(){
        let mut app = App::new();
        app
            .add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_valid_request_inner)
        ;
        app.update();
    }

    fn test_valid_request_inner(mut event_writer: EventWriter::<move_tile_event::SwitchTilesLogic>) {
        assert!( ! detected_as_invalid_request(
            basic_direction::BasicDirection::Up,
            &mut event_writer
        ));
        assert!(detected_as_invalid_request(
            basic_direction::BasicDirection::Right,
            &mut event_writer
        ));
        assert!(detected_as_invalid_request(
            basic_direction::BasicDirection::Down,
            &mut event_writer
        ));
        assert!( ! detected_as_invalid_request(
            basic_direction::BasicDirection::Left,
            &mut event_writer
        ));
    }

    fn detected_as_invalid_request(
        from_dir: basic_direction::BasicDirection,
        event_writer: &mut EventWriter::<move_tile_event::SwitchTilesLogic>
    )-> bool
    {
        let mut board=solved_board_builder::generate_solved_board(DEFAULT_BOARD_SIDE_LENGTH);
        board.ignore_player_input=false;
        let direction_check_outcome=
            move_into_empty_from_direction(
                event_writer, 
                &board,
                from_dir
            );

        println!("for {:?}, {:?}", from_dir, direction_check_outcome);

        match direction_check_outcome{
            Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(_))=> true,
            _=> false
        }
    }
}