use crate::logic::enums::board_building_request::BoardBuildingRequest;
use crate::prelude::*;

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    (
                        move_tiles_with_keyboard,
                        listen_for_board_reset,
                    ).run_if(in_state(AppState::Game)),
                    move_between_loader_screens.run_if(in_state(AppState::Loader)),
                    close_are_you_sure_message,
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
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

fn move_between_loader_screens(
    mut event_writer: EventWriter<LoaderScreenActionEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let screen_change_requests = keyboard_input
        .get_just_pressed()
        .map(BasicDirection::from_keycode);
    for valid_direction in screen_change_requests.flatten() {
        match valid_direction{
            BasicDirection::Right => {
                event_writer.send(LoaderScreenActionEvent{
                    action: LoaderScreenAction::ChangeScreen(ScreenChangeRequestType::Next)
                });
            },
            BasicDirection::Left => {
                event_writer.send(LoaderScreenActionEvent{
                    action: LoaderScreenAction::ChangeScreen(ScreenChangeRequestType::Previous)
                });
            },
            _ => {}
        }
    }
}

fn open_menu(
    mut menu_toggle_event_writer: EventWriter<ToggleMenu>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        menu_toggle_event_writer.send(ToggleMenu::default());
    }
}

fn close_are_you_sure_message(
    mut are_you_sure_action_event_writer: EventWriter<AreYouSureMessageButtonEvent>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        are_you_sure_action_event_writer.send(AreYouSureMessageButtonEvent{
            action: AreYouSureMessageButtonAction::Cancel
        });
    }
}

fn listen_for_board_reset(
    mut input_event_writer: EventWriter<BuildNewBoard>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        input_event_writer.send(BuildNewBoard(
            if keyboard_input.pressed(KeyCode::ShiftLeft){
                BoardBuildingRequest::CreateANewBoardFromNothing
            }else{
                BoardBuildingRequest::ShuffleExistingBoard
            }
        ));
    }
}

fn listen_for_app_closing(
    mut end_game_event_writer: EventWriter<EndGame>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) && keyboard_input.pressed(KeyCode::ShiftLeft) {
        end_game_event_writer.send(EndGame);
    }
}




fn listed_for_debug_key_which_is_k(
    
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyK) {
        
    }
}