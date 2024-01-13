use crate::{costume_event::{screen_changing_event, ui_event}, prelude::*};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Game,
}

/// tags should only be given to parents because visibility is inherited
#[derive(Component, Default, PartialEq, Eq, Debug)]
pub enum OnScreenTag {
    #[default]
    Menu,
    Game,
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
        screen_changing_event::ToggleVisibilityForElementsWithTag,
    >,
) {
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(OnScreenTag::Game));
}

fn toggle_board_lock(mut game_board_query: Query<&mut TileTypeBoard, With<GameBoard>>) {
    let current_lock_state = &mut game_board_query.single_mut().ignore_player_input;
    *current_lock_state = !*current_lock_state;
}

fn toggle_visibility_for_menu_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<
        screen_changing_event::ToggleVisibilityForElementsWithTag,
    >,
) {
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(OnScreenTag::Menu));
}

fn set_menu_indicators_to_fit_current(
    mut event_writer: EventWriter<screen_changing_event::SetPlannedPropertiesToFitCurrent>,
) {
    event_writer.send(SetPlannedPropertiesToFitCurrent);
}

fn toggle_menu_button(
    mut button_toggle_event_writer: EventWriter<ui_event::ToggleButton>,
    menu_toggle_button_entity: Query<Entity, With<MenuToggleButton>>,
){
    button_toggle_event_writer.send(ui_event::ToggleButton {
        entity: menu_toggle_button_entity.single(),
    });
}
