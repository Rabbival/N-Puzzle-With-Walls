use crate::{prelude::*, costume_event::{screen_changing_event, app_event}};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game
}

/// tags should only be given to parents because visibility is inherited
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
            .add_systems(Update, toggle_menu)
            .add_systems(
                OnExit(GameState::Game), (
                toggle_visibility_for_game_screen_elements.in_set(StateChangeSystemSets::StateChangeListening),
                toggle_board_lock
            ))
            .add_systems(
                OnEnter(GameState::Game), (
                toggle_visibility_for_game_screen_elements.in_set(StateChangeSystemSets::StateChangeListening),
                toggle_board_lock
            ))
            .add_systems(
                OnExit(GameState::Menu), (
                    toggle_visibility_for_menu_screen_elements,
                ).in_set(StateChangeSystemSets::StateChangeListening)
            )
            .add_systems(
                OnEnter(GameState::Menu), (
                    toggle_visibility_for_menu_screen_elements,
                    set_menu_indicators_to_fit_current
                ).in_set(StateChangeSystemSets::StateChangeListening)
            )
        ;
    }
}


fn toggle_menu(
    mut event_listener: EventReader<app_event::ToggleMenu>,
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
){
    for _ in event_listener.read(){
        match game_state.get() {
            GameState::Game => {
                next_state.set(GameState::Menu);
            },
            GameState::Menu => {
                next_state.set(GameState::Game);
            }
        }
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