use crate::logic::enums::board_building_request::BoardBuildingRequest;
use crate::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
	PendingSolvedBoardGeneration,
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
        app.init_state::<GameState>()
			.add_systems(
				Update,
					set_game_state_according_to_board_gen_request
						.in_set(InputSystemSets::InputHandling)
						.after(set_applied_props_and_exit_menu)
			);
	}
}

fn set_game_state_according_to_board_gen_request(
	mut event_reader: EventReader<BuildNewBoard>,
	mut game_state: ResMut<NextState<GameState>>,
){
	for board_gen_request in event_reader.read(){
		match board_gen_request.0{
			BoardBuildingRequest::CreateANewBoardFromNothing => {
				game_state.set(GameState::PendingSolvedBoardGeneration)
			},
			BoardBuildingRequest::ShuffleExistingBoard => {
				game_state.set(GameState::SolvedBoardGenerated)
			},
		}
	}
}
