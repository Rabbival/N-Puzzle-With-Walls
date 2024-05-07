use crate::prelude::*;

pub struct DataBaseRequestHandlerPlugin;

impl Plugin for DataBaseRequestHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                (listen_for_save_requests,
                save_to_data_base_and_system).chain(),
                remove_from_data_base_and_system,
                listen_to_db_clearing_request
            ).in_set(InputSystemSets::InputHandling));
    }
}

fn listen_for_save_requests(
    mut event_writer: EventWriter<LayoutSaveAttemptOutcomeEvent>,
    mut write_to_db_event_writer: EventWriter<SaveToDB>,
    mut event_reader: EventReader<SaveWallsLayoutButtonPressed>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    domain_boards_query: Query<(&DomainBoard, &DomainBoardName)>,
    current_board_wall_locations: Res<CurrentBoardWallLocations>,
    db_manager: Res<DataBaseManager>
){
    for _save_request in event_reader.read(){
        if db_manager.get_saved_layouts_of_all_difficulties_count() >= super::MAX_SAVED_LAYOUTS as usize {
            event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::WallsLayoutsAtCapacity));
        }
        else{
            let wall_locations = current_board_wall_locations.0.clone();
            if let Some(existing_board_name) = domain_board_already_exists(
                &domain_boards_query,
                &applied_board_props_query.single().size,
                &wall_locations
            ){
                event_writer.send(LayoutSaveAttemptOutcomeEvent(SaveAttemptOutcome::WallLayoutAlreadyExistsInMemory(existing_board_name)));
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

fn domain_board_already_exists(
    domain_boards_query: &Query<(&DomainBoard, &DomainBoardName)>,
    new_board_size: &BoardSize,
    new_wall_locations: &Vec<GridLocation>
) -> Option<ExistingWallLayoutName> {
    for (domain_board, domain_board_name) in domain_boards_query{
        if domain_board.board_props.size == *new_board_size
            && *new_wall_locations == domain_board.wall_locations {
            return Some(ExistingWallLayoutName(domain_board_name.0.clone()));
        }
    }
    None
}

fn save_to_data_base_and_system(
    mut event_writer: EventWriter<SuccessSavingToDB>,
    mut event_reader: EventReader<SaveToDB>,
    mut db_manager: ResMut<DataBaseManager>,
    domain_board_query: Query<(Entity, &DomainBoardName, &DomainBoard)>,
    mut commands: Commands
){
    for save_request in event_reader.read(){
        match save_to_data_base_and_system_inner(
            save_request,
            &mut db_manager,
            &domain_board_query,
            &mut commands
        ){
            Err(data_base_error) => {
                print_data_base_error(data_base_error);
            },
            Ok(index_saved_to) => {
                event_writer.send(SuccessSavingToDB(index_saved_to));
            }
        }
    }
}

fn save_to_data_base_and_system_inner(
    save_request: &SaveToDB,
    db_manager: &mut DataBaseManager,
    domain_board_query: &Query<(Entity, &DomainBoardName, &DomainBoard)>,
    commands: &mut Commands
) -> Result<SavedLayoutIndexInDifficultyVec, DataBaseError>
{
    let layout_content_string = ron::ser::to_string_pretty(
        &save_request.0, ron::ser::PrettyConfig::default()).unwrap();
    let layout_name_string = db_manager.generate_default_name_for_board().0;
    write_to_file(
        FolderToAccess::SavedLayouts,
        layout_name_string.clone(),
        layout_content_string
    ).unwrap();

    super::wrap_to_data_base_error(
        db_manager.insert_layout_and_spawn_entity(
            &DomainBoardName(layout_name_string),
            &save_request.0,
            domain_board_query,
            commands
        )
    )
}

fn remove_from_data_base_and_system(
    mut event_writer: EventWriter<SuccessRemovingFromDB>,
    mut event_reader: EventReader<RemoveFromDB>,
    mut db_manager: ResMut<DataBaseManager>,
    domain_board_name_query: Query<&DomainBoardName>,
    mut commands: Commands
){
    for removal_request in event_reader.read(){
        match remove_from_data_base_and_system_inner(
            &removal_request.0,
            db_manager.as_mut(),
            &domain_board_name_query,
            &mut commands
        ){
            Err(data_base_error) => {
                print_data_base_error(data_base_error);
            },
            Ok(_) => {
                event_writer.send(SuccessRemovingFromDB(removal_request.0));
            }
        }
    }
}

fn remove_from_data_base_and_system_inner(
    layout_index: &SavedLayoutIndexInDifficultyVec,
    db_manager: &mut DataBaseManager,
    domain_board_name_query: &Query<&DomainBoardName>,
    commands: &mut Commands
) -> Result<(), DataBaseError>
{
    match db_manager.remove_layout_by_index_and_despawn_entity(layout_index, domain_board_name_query, commands){
        Some(removed_domain_board_name)=> {
            if delete_text_file(
                FolderToAccess::SavedLayouts,
                removed_domain_board_name.0.clone()
            ).is_err()
            {
                Err(DataBaseError::SystemAccessError
                    (SystemAccessError::CouldntFindFile(
                        SystemFileName::from_name(
                            removed_domain_board_name.0, 
                            SystemFileType::TextFile
                        )
                    )))
            }else{
                Ok(())
            }
        },
        None => Ok(())
    }
}

fn listen_to_db_clearing_request(
    mut event_writer: EventWriter<SuccessClearingDB>,
    mut event_reader: EventReader<ClearDB>,
    mut db_manager: ResMut<DataBaseManager>
){
    for _clear_request in event_reader.read() {
        match clear_db(&mut db_manager){
            Err(data_base_error) => {
                print_data_base_error(data_base_error);
            },
            Ok(_) => {
                event_writer.send(SuccessClearingDB);
            }
        }
    }
}

fn clear_db(
    db_manager: &mut DataBaseManager
) -> Result<(), DataBaseError>
{
    db_manager.saved_layouts = HashMap::new();
    create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
    let valid_text_file_names =
        get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);

    for valid_text_file_name in valid_text_file_names{
        let valid_text_file_name_excluding_postfix =
            valid_text_file_name.to_name();
        let file_deletion_result = delete_text_file(
            FolderToAccess::SavedLayouts,
            String::from(valid_text_file_name_excluding_postfix)
        );
        if file_deletion_result.is_err(){
            return Err(DataBaseError::SystemAccessError
                (SystemAccessError::CouldntFindFile(valid_text_file_name)));
        }
    }
    Ok(())
}