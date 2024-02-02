use crate::{prelude::*, costume_event::{game_event, board_set_event}};
use crate::logic::board_props::update_board_properties;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
	PendingSolvedBoardGen,
	SolvedBoardGenerated,
	GameBoardGenerated,
	PostGameBoardGenerationChangesDone,
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
					set_game_state_according_to_board_gen_request
						.in_set(InputSystemSets::InputHandling)
						.after(update_board_properties::set_applied_props_and_begin_generation)
			)
			;
	}
}

fn toggle_victory(
	mut victory_message_toggle_writer: EventWriter<game_event::ToggleVictoryMessage>
){
	victory_message_toggle_writer.send(game_event::ToggleVictoryMessage);
}

fn set_game_state_according_to_board_gen_request(
	mut event_listener: EventReader<board_set_event::BuildNewBoard>,
	mut game_state: ResMut<NextState<GameState>>,
){
	for board_gen_request in event_listener.read(){
		if board_gen_request.reroll_solved{
			game_state.set(GameState::PendingSolvedBoardGen);
		}else{
			game_state.set(GameState::SolvedBoardGenerated);
		}
	}
}
