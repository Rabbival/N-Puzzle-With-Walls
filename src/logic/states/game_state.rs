use crate::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Game,
    Menu
}

/// tags should only be given to parents as we despawn recursively and visibility is inherited
#[derive(Component, Default, PartialEq, Eq, Debug)]
pub enum OnScreenTag{
    #[default]
    Game,
    Menu
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
            ))
        ;
    }
}


fn toggle_visibility_for_game_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<ToggleVisibilityForElementsWithTag>
){
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(OnScreenTag::Game));
}

fn toggle_board_lock(mut game_board_query: Query<&mut TileTypeBoard,With<GameBoard>>){
    let current_lock_state=&mut game_board_query.single_mut().ignore_player_input;
    *current_lock_state = ! *current_lock_state;
}

fn toggle_visibility_for_menu_screen_elements(
    mut visibility_toggle_event_writer: EventWriter<ToggleVisibilityForElementsWithTag>
){
    visibility_toggle_event_writer.send(ToggleVisibilityForElementsWithTag(OnScreenTag::Menu));
}