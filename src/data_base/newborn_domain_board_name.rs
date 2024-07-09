use crate::prelude::*;


pub const MAX_DOMAIN_BOARD_NAME_LENGTH: usize = 22;

#[derive(Resource, Default)]
pub struct NewbornDomainBoardName{
    pub optional_name: Option<DomainBoardName>,
    pub index_of_existing_board_with_name: Option<SavedLayoutIndexInDifficultyVec>
}

pub struct NewbornDomainBoardNamePlugin;

impl Plugin for NewbornDomainBoardNamePlugin{
    fn build(&self, app: &mut App) {
        app
            .init_resource::<NewbornDomainBoardName>()
            .add_systems(
                Update, (
                    show_suggested_newborn_board_name,
                    set_index_of_existing_board_with_name
                ).in_set(InputSystemSets::PostInitialChanges)
            );
    }
}

fn show_suggested_newborn_board_name(
    mut newborn_board_name_update_event_writer: EventWriter<UpdateNewbornDomainBoardName>,
    mut event_reader: EventReader<SetNewbornDomainBoardNameToDefault>,
    mut newborn_domain_board_name: ResMut<NewbornDomainBoardName>,
    game_board_name_query: Query<&DomainBoardName, With<GameBoard>>,
    board_name_query: Query<&DomainBoardName>,
    db_manager: Res<DataBaseManager>
){
    for _event in event_reader.read(){
        let game_board_name = game_board_name_query.single();
        let suggested_board_name = determine_suggested_board_name(
            &mut newborn_domain_board_name,
            game_board_name,
            &board_name_query,
            &db_manager
        );
        newborn_board_name_update_event_writer.send(
            UpdateNewbornDomainBoardName(suggested_board_name)
        );
    }
}

fn determine_suggested_board_name(
    newborn_domain_board_name: &mut NewbornDomainBoardName,
    game_board_name: &DomainBoardName,
    board_name_query: &Query<&DomainBoardName>,
    db_manager: &DataBaseManager
) -> DomainBoardName
{
    if *game_board_name != DomainBoardName::default() {
        newborn_domain_board_name.index_of_existing_board_with_name =
            db_manager.get_existing_board_name_index(
                game_board_name,
                board_name_query
            );
        game_board_name.clone()
    }else {
        db_manager.generate_unique_default_name_for_board(board_name_query)
    }
}

fn set_index_of_existing_board_with_name(
    mut event_reader: EventReader<UpdateNewbornDomainBoardName>,
    domain_board_names_query: Query<&DomainBoardName>,
    db_manager: Res<DataBaseManager>,
    mut newborn_domain_board_name: ResMut<NewbornDomainBoardName>
){
    for name_update_request in event_reader.read(){
        newborn_domain_board_name.index_of_existing_board_with_name = 
            db_manager.get_existing_board_name_index(
              &name_update_request.0,
              &domain_board_names_query
            );
    }
}