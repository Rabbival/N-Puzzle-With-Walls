use crate::{prelude::*, costume_event::game_event};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Regular,
    Victory,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
			.add_systems(
				OnEnter(GameState::Victory),
					toggle_victory,
			)
            .add_systems(
                OnExit(GameState::Victory),
					toggle_victory,
            );
	}
}

fn toggle_victory(
	mut victory_message_toggle_writer: EventWriter<game_event::ToggleVictoryMessage>
){
	victory_message_toggle_writer.send(game_event::ToggleVictoryMessage);
}

