use crate::{prelude::*, costume_event::screen_changing_event};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game
}

/// tags should only be given to parents as we despawn recursively and visibility is inherited
#[derive(Component, Default, PartialEq, Eq, Debug)]
pub enum OnScreenTag{
    #[default]
    Menu,
    Game
}


pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameState>()
            .add_systems(
                OnExit(GameState::Game), (
                toggle_visibility_for_game_screen_elements,
                toggle_board_lock
            ))
            .add_systems(
                OnEnter(GameState::Game), (
                toggle_visibility_for_game_screen_elements,
                toggle_board_lock
            ))
            .add_systems(
                OnExit(GameState::Menu), (
                toggle_visibility_for_menu_screen_elements,
            ))
            .add_systems(
                OnEnter(GameState::Menu), (
                toggle_visibility_for_menu_screen_elements,
                set_menu_indicators_to_fit_current
            ))
        ;
    }
}


fn toggle_visibility_for_game_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<screen_changing_event::ToggleVisibilityForElementsWithTag>
){
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(OnScreenTag::Game));
}

fn toggle_board_lock(mut game_board_query: Query<&mut TileTypeBoard,With<GameBoard>>){
    let current_lock_state=&mut game_board_query.single_mut().ignore_player_input;
    *current_lock_state = ! *current_lock_state;
}

fn toggle_visibility_for_menu_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<screen_changing_event::ToggleVisibilityForElementsWithTag>
){
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(OnScreenTag::Menu));
}

fn set_menu_indicators_to_fit_current(
    mut event_writer: EventWriter<screen_changing_event::SetPlannedPropertiesToFitCurrent>
){
    event_writer.send(SetPlannedPropertiesToFitCurrent);
}