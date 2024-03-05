use crate::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
    Builder,
    Loader
}

/// tags should only be given to parents because visibility is inherited
#[derive(Component, Default, PartialEq, Eq, Debug)]
pub enum CustomOnScreenTag {
    #[default]
    Menu,
    Game,
    Builder,
    Loader
}

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(
                OnExit(AppState::Game),
                (
                    toggle_visibility_for_game_screen_elements
                        .in_set(StateChangeSystemSets::StateChangeListening),
                    toggle_board_lock,
                ),
            )
            .add_systems(
                OnEnter(AppState::Game),
                (
                    toggle_visibility_for_game_screen_elements
                        .in_set(StateChangeSystemSets::StateChangeListening),
                    toggle_board_lock,
                ),
            )
            .add_systems(
                OnExit(AppState::Menu),
                (
                    toggle_visibility_for_menu_screen_elements,
                    toggle_menu_button
                )
                    .in_set(StateChangeSystemSets::StateChangeListening),
            )
            .add_systems(
                OnEnter(AppState::Menu),
                (
                    toggle_visibility_for_menu_screen_elements,
                    toggle_menu_button,
                    set_menu_indicators_to_fit_current,
                )
                    .in_set(StateChangeSystemSets::StateChangeListening),
            );
    }
}

fn toggle_visibility_for_game_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<
        ToggleVisibilityForElementsWithTag,
    >,
) {
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(CustomOnScreenTag::Game));
}

fn toggle_board_lock(mut game_board_query: Query<&mut TileBoard, With<GameBoard>>) {
    let current_lock_state = &mut game_board_query.single_mut().ignore_player_input;
    *current_lock_state = !*current_lock_state;
}

fn toggle_visibility_for_menu_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<
        ToggleVisibilityForElementsWithTag,
    >,
) {
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(CustomOnScreenTag::Menu));
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
