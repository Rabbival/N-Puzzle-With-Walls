use crate::prelude::*;
use crate::system::system_access::create_folder_if_none_exists_yet;

pub struct DataBaseStartupPlugin;

impl Plugin for DataBaseStartupPlugin{
	fn build(&self, app: &mut App) {
		app
			.add_systems(PreStartup, (
					read_saved_layout_from_system,
					insert_saved_layout_entities_to_data_base
				).chain()
			);
	}
}

fn read_saved_layout_from_system(mut commands: Commands){
	create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
	let valid_text_file_names =
		get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);
	
	for valid_text_file_name in valid_text_file_names{
		match domain_board_from_file(
			FolderToAccess::SavedLayouts,
			valid_text_file_name.clone()
		){
			Err(system_error) => {
				print_system_access_error(system_error);
				continue;
			},
			Ok(parsed_domain_board) => {
				let board_name = DomainBoardName(valid_text_file_name.to_name());
				if parsed_domain_board.has_valid_counters_and_size(){
					DataBaseManager::spawn_layout_entity(
						&board_name,
						&parsed_domain_board,
						&mut commands
					);
				}else{
					print_data_base_error(
						DataBaseError::MismatchedGridAndProperties(board_name)
					);
					continue;
				}
			}
		}
	}
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