use crate::prelude::*;
use crate::system::system_access::create_folder_if_none_exists_yet;


#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: Vec<DomainBoard>,
}

pub struct DataBaseManagerPlugin;

impl Plugin for DataBaseManagerPlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_systems(Startup, read_system_text_files_into_db)
			.add_systems(Update, (
				save_to_data_base_and_system,
				remove_from_data_base_and_system,
				listen_to_db_clearing_request
			));
	}
}

fn read_system_text_files_into_db(
	mut db_manager: ResMut<DataBaseManager>
){
	if let Err(system_access_error) = read_system_text_files_into_db_inner(db_manager.as_mut()){
		print_system_access_error(system_access_error);
	}
}

fn read_system_text_files_into_db_inner(
	db_manager: &mut DataBaseManager
) -> Result<(), SystemAccessError>
{
	create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
	let valid_text_file_names =
		get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);

	for valid_text_file_name in valid_text_file_names{
		let domain_board = domain_board_from_file(
			FolderToAccess::SavedLayouts,
			valid_text_file_name
		)?;
		
		//TODO: if you choose to validate board quality, do it here
		
		db_manager.insert_layout(&domain_board);
	}
	Ok(())
}

fn save_to_data_base_and_system(
	mut event_listener: EventReader<SaveToDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	for save_request in event_listener.read(){
		let layout_content_string = ron::ser::to_string_pretty(
			&save_request.0, ron::ser::PrettyConfig::default()).unwrap();
		let layout_name_string = db_manager.generate_default_name_for_board();
		write_to_file(
			FolderToAccess::SavedLayouts,
			layout_name_string.clone(),
			layout_content_string
		).unwrap();

		db_manager.as_mut().insert_layout(&save_request.0);
	}
}

fn remove_from_data_base_and_system(
	mut event_listener: EventReader<RemoveFromDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	for removal_request in event_listener.read(){
		if let Err(system_access_error) =
			remove_from_data_base_and_system_inner(&removal_request.0, db_manager.as_mut())
		{
			print_system_access_error(system_access_error);
		}
	}
}

fn remove_from_data_base_and_system_inner(
	layout_index: &SavedLayoutIndex,
	db_manager: &mut DataBaseManager
) -> Result<(), SystemAccessError>
{
	match db_manager.remove_layout_by_index(layout_index){
		Some(removed_domain_board)=> {
			if delete_text_file(
				FolderToAccess::SavedLayouts,
				removed_domain_board.board_name.clone()
			).is_err()
			{
				Err(SystemAccessError::CouldntFindFile(FileName(removed_domain_board.board_name.clone())))
			}else{
				Ok(())
			}
		},
		None => Ok(())
	}
}

fn listen_to_db_clearing_request(
	mut event_listener: EventReader<ClearDB>,
	db_manager: ResMut<DataBaseManager>
){
	if let Err(system_access_error) =
		listen_to_db_clearing_request_inner(
			&mut event_listener,
			db_manager
		){
		print_system_access_error(system_access_error);
	}
}

fn listen_to_db_clearing_request_inner(
	event_listener: &mut EventReader<ClearDB>,
	mut db_manager: ResMut<DataBaseManager>
) -> Result<(), SystemAccessError>
{
	for _clear_request in event_listener.read(){
		db_manager.saved_layouts = vec!();
		create_folder_if_none_exists_yet(FolderToAccess::SavedLayouts);
		let valid_text_file_names =
			get_all_valid_text_file_names_in_folder(FolderToAccess::SavedLayouts);

		for valid_text_file_name in valid_text_file_names{
			let valid_text_file_name_excluding_postfix =
				&valid_text_file_name[..valid_text_file_name.len()-4];
			let file_deletion_result = delete_text_file(
				FolderToAccess::SavedLayouts,
				String::from(valid_text_file_name_excluding_postfix)
			);
			if file_deletion_result.is_err(){
				return Err(SystemAccessError::CouldntFindFile(FileName(valid_text_file_name)));
			}
		}
	}
	Ok(())
}

impl DataBaseManager{
	pub fn insert_layout(&mut self, layout: &DomainBoard){
		self.saved_layouts.push(layout.clone());
	}

	pub fn remove_layout_by_index(&mut self, index: &SavedLayoutIndex) -> Option<DomainBoard>{
		let index_value = index.0;
		if index_value < self.saved_layouts.len(){
			Some(self.saved_layouts.swap_remove(index.0))	
		}else{
			None
		}
	}

	pub fn get_saved_layouts_ref(&self) -> &Vec<DomainBoard>{
		&self.saved_layouts
	}
	
	pub fn generate_default_name_for_board(&self) -> String {
		format!("layout_{:?}", self.saved_layouts.len())
	}
}