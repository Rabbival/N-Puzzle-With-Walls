use crate::{prelude::*, costume_event::{ui_event, board_set_event}};

pub struct VictoryUiLogicPlugin;

impl Plugin for VictoryUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_victory_button_press)
                .in_set(InputSystemSets::ChangesBasedOnInput),
        );
    }
}

fn listen_for_victory_button_press(
	mut button_event_listener: EventReader<ui_event::GameButtonPressed>,
	mut spawn_board_event_writer: EventWriter<board_set_event::BuildNewBoard>,
	mut set_game_state_to_regular: ResMut<NextState<GameState>>,
){
	for button_event in button_event_listener.read(){
		match button_event.action{
			GameButtonAction::ResetBoard => {
				spawn_board_event_writer.send(board_set_event::BuildNewBoard {
					reroll_solved: false,
				});
				set_game_state_to_regular.set(GameState::Regular);
			}
		}
	}
}