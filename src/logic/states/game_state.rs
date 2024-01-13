use crate::{prelude::*, costume_event::{game_event, board_set_event}};

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
            )
			.add_systems(
				Update,
				cancel_victory_state_when_board_rerolls
			)
			;
	}
}

fn toggle_victory(
	mut victory_message_toggle_writer: EventWriter<game_event::ToggleVictoryMessage>
){
	victory_message_toggle_writer.send(game_event::ToggleVictoryMessage);
}

fn cancel_victory_state_when_board_rerolls(
	mut spawn_board_event_listener: EventReader<board_set_event::BuildNewBoard>,
	mut set_game_state_to_regular: ResMut<NextState<GameState>>,
){
	for _new_board_request in spawn_board_event_listener.read(){
		set_game_state_to_regular.set(GameState::Regular);	
	}
}
