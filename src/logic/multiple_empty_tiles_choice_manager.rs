use crate::prelude::*;

#[derive(Debug, Resource, Default, Clone)]
pub struct MultipleEmptyTilesChoiceManager{
    pub choice_pending: bool,
    pub possible_empty_tiles_locations_and_directions: Option<HashMap<BasicDirection, Tile>>,
}

pub struct MultipleEmptyTilesChoiceManagerPlugin;

impl Plugin for MultipleEmptyTilesChoiceManagerPlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<MultipleEmptyTilesChoiceManager>()
            .add_systems(Update, listen_for_set_requests.run_if(in_state(AppState::Game)))
            .add_systems(OnEnter(GameState::GameBoardGenerated), cancel_choice_pending)
        ;
    }
}

fn listen_for_set_requests(
    mut event_reader: EventReader<SetMultipleEmptyTilesChoiceManager>,
    mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>
){
    for set_request in event_reader.read(){
        *multiple_empty_tiles_choice_manager = set_request.new_config.clone();
    }
}

fn cancel_choice_pending(mut event_writer: EventWriter<SetMultipleEmptyTilesChoiceManager>){
    event_writer.send(SetMultipleEmptyTilesChoiceManager {
        new_config: MultipleEmptyTilesChoiceManager{
            choice_pending: false,
            possible_empty_tiles_locations_and_directions: None
        }
    });
}