use bevy::app::AppExit;

use crate::prelude::*;

pub struct EternalUiLogicPlugin;

impl Plugin for EternalUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_menu, listen_for_app_close_request)
                .in_set(InputSystemSets::InitialChanges),
        );
    }
}

fn toggle_menu(
    mut event_reader: EventReader<ToggleMenu>,
    game_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    applied_board_prop_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
) {
    for _ in event_reader.read() {
        let current_not_menu_state = 
            applied_board_prop_query.single().generation_method.to_app_state();
        match game_state.get() {
            AppState::Menu => {
                next_state.set(current_not_menu_state);
            }
            _ => {
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
