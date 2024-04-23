use crate::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    Builder,
    Loader
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(
                OnExit(AppState::Game),
                    toggle_board_lock,
            )
            .add_systems(
                OnEnter(AppState::Game),
                    toggle_board_lock
            )
            .add_systems(
                OnExit(AppState::Menu),
                    toggle_menu_button
                        .in_set(StateChangeSystemSets::StateChangeListening),
            )
            .add_systems(
                OnEnter(AppState::Menu),
                (
                    toggle_menu_button,
                    set_menu_indicators_to_fit_current,
                )
                    .in_set(StateChangeSystemSets::StateChangeListening),
            );
    }
}

fn toggle_board_lock(mut game_board_query: Query<&mut TileBoard, With<GameBoard>>) {
    let current_lock_state = &mut game_board_query.single_mut().ignore_player_input;
    *current_lock_state = !*current_lock_state;
}

fn set_menu_indicators_to_fit_current(
    mut event_writer: EventWriter<SetPlannedPropertiesToFitCurrent>,
) {
    event_writer.send(SetPlannedPropertiesToFitCurrent);
}

fn toggle_menu_button(
    mut button_toggle_event_writer: EventWriter<ToggleButton>,
    menu_toggle_button_entity: Query<Entity, With<MenuToggleButton>>,
){
    button_toggle_event_writer.send(ToggleButton {
        entity: menu_toggle_button_entity.single(),
    });
}
