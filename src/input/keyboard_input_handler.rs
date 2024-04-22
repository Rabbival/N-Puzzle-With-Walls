use crate::prelude::*;

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    move_tiles_with_keyboard.run_if(in_state(AppState::Game)),
                    listen_for_reset,
                    open_menu,


                    listed_for_debug_key_which_is_k


                )
                    .chain()
                    .in_set(InputSystemSets::InputListening),
                listen_for_app_closing,
            ),
        );
    }
}

fn move_tiles_with_keyboard(
    mut logic_event_writer: EventWriter<SwitchTilesLogic>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let move_requests = keyboard_input
        .get_just_pressed()
        .map(MoveRequest::new);
    for request in move_requests {
        if request.move_neighbor_from_direction.is_none() || request.empty_tile_index.is_none() {
            continue;
        } else {
            logic_event_writer.send(SwitchTilesLogic {
                move_neighbor_from_direction: request.move_neighbor_from_direction.unwrap(),
                empty_tile_index: request.empty_tile_index.unwrap(),
            });
        }
    }
}

fn open_menu(
    mut menu_toggle_event_writer: EventWriter<ToggleMenu>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        menu_toggle_event_writer.send(ToggleMenu)
    }
}

/// resets the solved board if shift is pressed too
fn listen_for_reset(
    mut input_event_writer: EventWriter<BuildNewBoard>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        let reroll_solved = keyboard_input.pressed(KeyCode::ShiftLeft);
        input_event_writer.send(BuildNewBoard { reroll_solved });
    }
}

fn listen_for_app_closing(
    mut end_game_event_writer: EventWriter<EndGame>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        end_game_event_writer.send(EndGame);
    }
}




fn listed_for_debug_key_which_is_k(
    mut event_writer: EventWriter<ClearDB>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::K) {
        event_writer.send(ClearDB);
    }
}