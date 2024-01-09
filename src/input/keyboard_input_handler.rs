use crate::{prelude::*, costume_event::{board_set_event, move_tile_event, app_event}};

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

fn move_tiles_with_keyboard(
    mut logic_event_writer: EventWriter<move_tile_event::SwitchTilesLogic>,
    keyboard_input: Res<Input<KeyCode>>,
){
    let move_requests 
        = keyboard_input.get_just_pressed()
        .map(|keycode|{
            MoveRequest::new(keycode)
        });
    for request in move_requests {
        if request.move_neighbor_from_direction.is_none() || request.empty_tile_index.is_none(){
            continue;
        }else {
            logic_event_writer.send(move_tile_event::SwitchTilesLogic{
                move_neighbor_from_direction: request.move_neighbor_from_direction.unwrap(), 
                empty_tile_index: request.empty_tile_index.unwrap(),
            });
        }
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

/// resets the solved board if shift is pressed too
fn listen_for_reset(
    mut input_event_writer: EventWriter<board_set_event::BuildNewBoard>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::R){
        let reroll_solved = keyboard_input.pressed(KeyCode::ShiftLeft);
        input_event_writer.send(board_set_event::BuildNewBoard{
            reroll_solved
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