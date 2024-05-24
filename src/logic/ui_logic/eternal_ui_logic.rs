use bevy::app::AppExit;

use crate::prelude::*;

#[derive(Resource, Default)]
pub struct StateBeforeMenuOpened(pub Option<AppState>);

pub struct EternalUiLogicPlugin;

impl Plugin for EternalUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<StateBeforeMenuOpened>()
            .add_systems(
                OnExit(AppState::Menu),
                toggle_menu_button
            )
            .add_systems(
                OnEnter(AppState::Menu),
                (
                    toggle_menu_button,
                )
            )
            .add_systems(
            Update,
            (toggle_menu, listen_for_app_close_request)
                .in_set(InputSystemSets::InitialChanges),
        );
    }
}

fn toggle_menu_button(
    mut button_toggle_event_writer: EventWriter<ToggleButton>,
    menu_toggle_button_entity: Query<Entity, With<MenuToggleButton>>,
){
    button_toggle_event_writer.send(ToggleButton {
        entity: menu_toggle_button_entity.single(),
    });
}

fn toggle_menu(
    mut event_reader: EventReader<ToggleMenu>,
    mut state_before_menu_opened: ResMut<StateBeforeMenuOpened>,
    app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for menu_toggle_event in event_reader.read() {
        let current_app_state = app_state.get();
        match current_app_state {
            AppState::Menu => {
                if let Some(pre_set_next_state) = menu_toggle_event.out_of_menu_into{
                    next_state.set(pre_set_next_state);
                }else if let Some(previous_app_state) = state_before_menu_opened.0{
                    next_state.set(previous_app_state);
                }
            }
            _ => {
                state_before_menu_opened.0 = Some(*current_app_state);
                next_state.set(AppState::Menu);
            }
        }
    }
}

fn listen_for_app_close_request(
    mut end_game_listener: EventReader<EndGame>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for _ in end_game_listener.read() {
        app_exit_events.send(AppExit);
    }
}
