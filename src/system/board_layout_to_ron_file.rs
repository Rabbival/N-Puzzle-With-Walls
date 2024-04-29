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
    mut event_reader: EventReader<SaveWallsLayoutButtonPressed>,
    mut write_to_db_event_writer: EventWriter<SaveToDB>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    domain_boards_query: Query<&DomainBoard>,
    current_board_wall_locations: Res<CurrentBoardWallLocations>,
    db_manager: Res<DataBaseManager>
){
    for _save_request in event_reader.read(){
        let saved_layout_reference = 
            db_manager.get_saved_layouts_ref();
        if saved_layout_reference.len() >= BOARD_LAYOUT_CAPACITY as usize {
            event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::WallsLayoutsAtCapacity));
        }
        else{
            let wall_locations = current_board_wall_locations.0.clone();
            if let Some(existing_board_name) = domain_board_exists_in_db(
                &domain_boards_query,
                &applied_board_props_query.single().size,
                &wall_locations
            ){
                event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::WallLayoutAlreadyExistsInMemory(existing_board_name)));
            }else{
                write_to_db_event_writer.send(SaveToDB(DomainBoard{
                    board_name: db_manager.generate_default_name_for_board(),
                    board_props: *applied_board_props_query.single(),
                    wall_locations
                }));
                event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::LayoutSavedSuccessfully));
            }
        }
    }
}

fn domain_board_exists_in_db(
    domain_boards_query: &Query<&DomainBoard>,
    new_board_size: &BoardSize,
    new_wall_locations: &Vec<GridLocation>
) -> Option<ExistingWallLayoutName> {
    for domain_board in domain_boards_query.iter(){
        if domain_board.board_props.size == *new_board_size
            && *new_wall_locations == domain_board.wall_locations {
            return Some(ExistingWallLayoutName(domain_board.board_name.0.clone()));
        }
    }
    None
}