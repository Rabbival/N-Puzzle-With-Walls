use crate::prelude::*;

pub const BOARD_LAYOUT_CAPACITY : u8 = 255;

pub struct BoardLayoutToRonFilePlugin;

impl Plugin for BoardLayoutToRonFilePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_save_requests);
    }
}

fn listen_for_save_requests(
    mut event_writer: EventWriter<LayoutSaveAttemptOutcomeEvent>,
    mut event_listener: EventReader<SaveWallsLayoutButtonPressed>,
    mut write_to_db_event_writer: EventWriter<SaveToDB>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    current_board_wall_locations: Res<CurrentBoardWallLocations>,
    db_manager: Res<DataBaseManager>
){
    for _save_request in event_listener.read(){
        let saved_layout_reference = 
            db_manager.get_saved_layouts_ref();
        if saved_layout_reference.len() >= BOARD_LAYOUT_CAPACITY as usize {
            event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::WallsLayoutsAtCapacity));
        }
        else{
            let wall_locations = current_board_wall_locations.0.clone();
            if board_exists_in_db(
                saved_layout_reference,
                &applied_board_props_query.single().size,
                &wall_locations
            ){
                event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::WallLayoutAlreadyExistsInMemory));
            }else{
                write_to_db_event_writer.send(SaveToDB(DomainBoard{
                    board_props: *applied_board_props_query.single(),
                    wall_locations
                }));
                event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::LayoutSavedSuccessfully));
            }
        }
    }
}

fn board_exists_in_db(
    saved_layouts: &HashMap<DomainBoardNameWithoutPostfix, DomainBoard>,
    new_board_size: &BoardSize,
    new_wall_locations: &Vec<GridLocation>
) -> bool {
    for saved_layout in saved_layouts{
        if saved_layout.1.board_props.size == *new_board_size
            && *new_wall_locations == saved_layout.1.wall_locations {
            return true;
        }
    }
    false
}