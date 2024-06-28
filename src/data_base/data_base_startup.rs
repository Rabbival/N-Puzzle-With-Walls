use crate::prelude::*;
use crate::system::system_access::create_folder_if_none_exists_yet;

pub struct DataBaseStartupPlugin;

impl Plugin for DataBaseStartupPlugin{
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, (
					read_saved_layout_from_system,
					insert_saved_layout_entities_to_data_base
				).chain()
			);
	}
}

fn read_saved_layout_from_system(mut commands: Commands){
	if let Err(data_base_error) = read_saved_layout_from_system_inner(&mut commands) {
		print_data_base_error(data_base_error);
	}
}

fn read_saved_layout_from_system_inner(commands: &mut Commands) -> Result<(), DataBaseError> {
	create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
	let valid_text_file_names =
		get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);
	
	for valid_text_file_name in valid_text_file_names{
		match domain_board_from_file(
			FolderToAccess::SavedLayouts,
			valid_text_file_name.clone()
		){
			Err(system_access_error) => {
				return Err(DataBaseError::SystemAccessError(system_access_error));
			},
			Ok(parsed_domain_board) => {
				let file_name_without_postfix = valid_text_file_name.to_name();
				DataBaseManager::spawn_layout_entity(
					&DomainBoardName(file_name_without_postfix),
					&parsed_domain_board,
					commands
				);
			}
		}
	}
	Ok(())
}

fn insert_saved_layout_entities_to_data_base(
	mut db_manager: ResMut<DataBaseManager>,
	domain_board_query: Query<(Entity, &DomainBoardName, &DomainBoard)>,
){
	for (entity, domain_board_name, domain_board)
		in &domain_board_query
	{
		db_manager.insert_layout(
			entity,
			domain_board_name,
			&domain_board.board_props.board_difficulty,
			&domain_board_query,
		);
	}
}