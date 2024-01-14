use crate::{prelude::*, costume_event::{ui_event, board_set_event}};

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
	mut button_event_listener: EventReader<ui_event::VictoryButtonPressed>,
	mut spawn_board_event_writer: EventWriter<board_set_event::BuildNewBoard>,
){
	for button_event in button_event_listener.read(){
		match button_event.action{
			VictoryButtonAction::ResetBoard => {
				spawn_board_event_writer.send(board_set_event::BuildNewBoard {
					reroll_solved: false,
				});
			}
		}
	}
}