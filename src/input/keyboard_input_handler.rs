use crate::{prelude::*, logic::enums::basic_direction, output::{print_to_console, error_handler}, costume_event::{board_set_event, move_tile_event, app_event}};

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ((
                        move_tiles_with_keyboard.run_if(in_state(GameState::Game)),
                        listen_for_reset, 
                        open_menu
                    )
                    .chain()
                    .in_set(InputSystemSets::InputListening),
                    
                    listen_for_app_closing
                ))
            ;
    }
}

fn open_menu(
    mut menu_toggle_event_writer: EventWriter<app_event::ToggleMenu>,
    keyboard_input: Res<Input<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::Space){
        menu_toggle_event_writer.send(app_event::ToggleMenu)
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
    if move_request_direction.is_none()  {
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
    mut input_event_writer: EventWriter<board_set_event::BuildNewBoard>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::R){

        input_event_writer.send(board_set_event::BuildNewBoard{
            reroll_solved: false
        });
    }
}

fn listen_for_app_closing(
    mut end_game_event_writer: EventWriter<app_event::EndGame>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::Escape){
        end_game_event_writer.send(app_event::EndGame);
    }
}


#[cfg(test)]
mod tests {
    use crate::logic::board_building::solved_board_builder;

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
        let mut board
            =solved_board_builder::generate_solved_board(&BoardProperties::default()).unwrap();
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