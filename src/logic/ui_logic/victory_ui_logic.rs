use crate::logic::enums::board_building_request::BoardBuildingRequest;
use crate::prelude::*;

pub struct VictoryUiLogicPlugin;

impl Plugin for VictoryUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_victory_button_press)
				.run_if(in_state(GameState::Victory))
                .in_set(InputSystemSets::InitialChanges),
        );
    }
}

fn listen_for_victory_button_press(
	mut button_event_reader: EventReader<VictoryButtonPressed>,
	mut spawn_board_event_writer: EventWriter<BuildNewBoard>,
){
	for button_event in button_event_reader.read(){
		match button_event.action{
			VictoryButtonAction::ResetBoard => {
				spawn_board_event_writer.send(BuildNewBoard(BoardBuildingRequest::ShuffleExistingBoard));
			}
		}
	}
}