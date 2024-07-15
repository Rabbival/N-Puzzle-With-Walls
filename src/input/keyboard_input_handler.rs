use crate::logic::enums::board_building_request::BoardBuildingRequest;
use crate::prelude::*;

#[derive(Resource, Debug, Default)]
struct TimePressed(pub HashMap<KeyCode, f32>);
const TIME_THRESHOLD_TO_CONTINUOUS_PRESS: f32 = 0.6;

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TimePressed>().add_systems(
            Update,
            (
                (
                    (
                        move_tiles_with_keyboard,
                        listen_for_board_reset,
                        listen_to_keyboard_typing_for_newborn_domain_board_name,
                    )
                        .run_if(in_state(AppState::Game)),
                    move_between_loader_screens.run_if(in_state(AppState::Loader)),
                    confirm_pop_up_message,
                    cancel_pop_up_message,
                    open_menu,
                    listed_for_debug_key_which_is_k,
                )
                    .chain()
                    .in_set(InputSystemSets::InputListening),
                listen_for_app_closing,
            ),
        );
    }
}

fn listen_to_keyboard_typing_for_newborn_domain_board_name(
    mut event_writer: EventWriter<KeyboardKeyTypedEvent>,
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut time_pressed: ResMut<TimePressed>,
    time: Res<Time>,
) {
    if let Visibility::Hidden = *pop_up_message_visibility.single() {
        return;
    }

    for just_key_pressed in keyboard_input.get_just_pressed() {
        send_pressed_key_event(just_key_pressed, &mut event_writer, &keyboard_input);
        if !time_pressed.0.contains_key(just_key_pressed) {
            time_pressed.0.insert(*just_key_pressed, 0.0);
        }
    }
    for key_released in keyboard_input.get_just_released() {
        time_pressed.0.remove(key_released);
    }
    for key_pressed in keyboard_input.get_pressed() {
        if let Some(key_time_pressed) = time_pressed.0.get_mut(key_pressed) {
            *key_time_pressed += time.delta_seconds();
            if *key_time_pressed > TIME_THRESHOLD_TO_CONTINUOUS_PRESS {
                send_pressed_key_event(key_pressed, &mut event_writer, &keyboard_input);
            }
        }
    }
}

fn send_pressed_key_event(
    key_code: &KeyCode,
    event_writer: &mut EventWriter<KeyboardKeyTypedEvent>,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
) {
    let shift_is_pressed = keyboard_input.pressed(KeyCode::ShiftLeft);
    event_writer.send(KeyboardKeyTypedEvent {
        keycode: *key_code,
        shift_pressed: shift_is_pressed,
    });
}

fn move_tiles_with_keyboard(
    mut logic_event_writer: EventWriter<SwitchTilesLogic>,
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Visibility::Inherited = *pop_up_message_visibility.single() {
        return;
    }

    let move_requests = keyboard_input.get_just_pressed().map(MoveRequest::new);
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
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Visibility::Inherited = *pop_up_message_visibility.single() {
        return;
    }

    let screen_change_requests = keyboard_input
        .get_just_pressed()
        .map(BasicDirection::from_keycode);
    for valid_direction in screen_change_requests.flatten() {
        match valid_direction {
            BasicDirection::Right => {
                event_writer.send(LoaderScreenActionEvent {
                    action: LoaderScreenAction::ChangeScreen(ScreenChangeRequestType::Next),
                });
            }
            BasicDirection::Left => {
                event_writer.send(LoaderScreenActionEvent {
                    action: LoaderScreenAction::ChangeScreen(ScreenChangeRequestType::Previous),
                });
            }
            _ => {}
        }
    }
}

fn listen_for_board_reset(
    mut input_event_writer: EventWriter<BuildNewBoard>,
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Visibility::Inherited = *pop_up_message_visibility.single() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        input_event_writer.send(BuildNewBoard(
            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                BoardBuildingRequest::CreateANewBoardFromNothing
            } else {
                BoardBuildingRequest::ShuffleExistingBoard
            },
        ));
    }
}

fn cancel_pop_up_message(
    mut pop_up_action_event_writer: EventWriter<PopUpMessageButtonEvent>,
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Visibility::Hidden = *pop_up_message_visibility.single() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        pop_up_action_event_writer.send(PopUpMessageButtonEvent {
            action: PopUpMessageButtonAction::Cancel,
        });
    }
}

fn confirm_pop_up_message(
    mut pop_up_action_event_writer: EventWriter<PopUpMessageButtonEvent>,
    confirm_allowed_query: Query<&ConfirmAllowed, With<PopUpMessageType>>,
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Visibility::Hidden = *pop_up_message_visibility.single() {
        return;
    }
    if !confirm_allowed_query.single().0 {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Enter)
        || keyboard_input.just_pressed(KeyCode::NumpadEnter)
    {
        pop_up_action_event_writer.send(PopUpMessageButtonEvent {
            action: PopUpMessageButtonAction::Confirm,
        });
    }
}

fn open_menu(
    mut menu_toggle_event_writer: EventWriter<ToggleMenu>,
    pop_up_message_visibility: Query<&Visibility, With<PopUpMessageType>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Visibility::Hidden = *pop_up_message_visibility.single() {
        if keyboard_input.just_pressed(KeyCode::Space) || keyboard_input.just_pressed(KeyCode::KeyM)
        {
            menu_toggle_event_writer.send(ToggleMenu::default());
        }
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

fn listed_for_debug_key_which_is_k(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyK) {}
}
