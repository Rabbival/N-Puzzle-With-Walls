use bevy::app::AppExit;

use crate::{
    costume_event::app_event,
    prelude::*,
};

pub struct EternalUiLogicPlugin;

impl Plugin for EternalUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_menu, listen_for_app_close_request)
                .in_set(InputSystemSets::ChangesBasedOnInput),
        );
    }
}

fn toggle_menu(
    mut event_listener: EventReader<app_event::ToggleMenu>,
    game_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _ in event_listener.read() {
        match game_state.get() {
            AppState::Game => {
                next_state.set(AppState::Menu);
            }
            AppState::Menu => {
                next_state.set(AppState::Game);
            }
        }
    }
}

fn listen_for_app_close_request(
    mut end_game_listener: EventReader<app_event::EndGame>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for _ in end_game_listener.read() {
        app_exit_events.send(AppExit);
    }
}
