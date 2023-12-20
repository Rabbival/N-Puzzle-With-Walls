use crate::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game
}

#[derive(Component, Default)]
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
                OnExit(GameState::Game), 
                remove_on_game_screen_tagged
            )
        ;
    }
}

fn remove_on_game_screen_tagged(){
    //send an event to graphics
}