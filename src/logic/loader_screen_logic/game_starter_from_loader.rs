use crate::prelude::*;

pub struct GameStarterFromLoaderPlugin;

impl Plugin for GameStarterFromLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.
            add_systems(
                Update,
                listen_to_game_start_from_loader_requests
            );
    }
}

fn listen_to_game_start_from_loader_requests(
    mut spawn_board_event_writer: EventWriter<BuildNewBoard>,
    mut event_reader: EventReader<LoaderScreenActionInitiated>,
    mut current_board_wall_locations: ResMut<CurrentBoardWallLocations>,
    data_base_manager: Res<DataBaseManager>,
    mut applied_board_props_query: Query<&mut BoardProperties, With<AppliedBoardProperties>>,
){
    for loader_action in event_reader.read(){
        if let LoaderScreenAction::GenerateBoard(optional_saved_layout_index) = 
            loader_action.action
        {
            if let Some(chosen_layout_screen_and_slot) = optional_saved_layout_index {
                let optional_chosen_domain_board = data_base_manager.try_get_layout_ref(
                    &chosen_layout_screen_and_slot
                );
                if let Some(chosen_domain_board) = optional_chosen_domain_board {
                    current_board_wall_locations.0 = chosen_domain_board.wall_locations.clone();
                    let mut board_propertied_ref = applied_board_props_query.single_mut();
                    board_propertied_ref.size = chosen_domain_board.board_props.size;
                    board_propertied_ref.wall_count = chosen_domain_board.board_props.wall_count;
                    board_propertied_ref.empty_count = chosen_domain_board.board_props.empty_count;
                    spawn_board_event_writer.send(BuildNewBoard{ build_new_solved_board: true });
                }
            }
        }
    }
}