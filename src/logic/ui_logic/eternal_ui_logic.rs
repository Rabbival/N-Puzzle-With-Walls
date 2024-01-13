use bevy::app::AppExit;

use crate::{
    costume_event::{app_event, ui_event},
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
    mut button_toggle_event_writer: EventWriter<ui_event::ToggleButton>,
    menu_toggle_button_entity: Query<Entity, With<MenuToggleButton>>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in event_listener.read() {
        button_toggle_event_writer.send(ui_event::ToggleButton {
            entity: menu_toggle_button_entity.single(),
        });
        match game_state.get() {
            GameState::Game => {
                next_state.set(GameState::Menu);
            }
            GameState::Menu => {
                next_state.set(GameState::Game);
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
