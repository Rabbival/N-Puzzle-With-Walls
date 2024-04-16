use crate::prelude::*;

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
    current_board_wall_locations: Res<CurrentBoardWallLocations>
){
    for _save_request in event_listener.read(){
        let wall_locations = current_board_wall_locations.0.clone();
        write_to_db_event_writer.send(SaveToDB(DomainBoard{
            board_props: *applied_board_props_query.single(),
            wall_locations
        }));
        
        //until there's logic
        event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::LayoutSavedSuccessfully));
    }
}