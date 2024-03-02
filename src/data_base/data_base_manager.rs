use crate::prelude::*;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct DomainBoardNameWithoutPostfix(pub String);

#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: HashMap<DomainBoardNameWithoutPostfix, DomainBoard>,
}

pub struct DataBaseManagerPlugin;

impl Plugin for DataBaseManagerPlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_systems(Startup, read_system_text_files_into_db)
			.add_systems(Update, (
				save_to_data_base_and_system,
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
		let valid_text_file_name_excluding_postfix =
			&valid_text_file_name.clone()[..valid_text_file_name.len()-4];
		let domain_board = domain_board_from_file(
			FolderToAccess::SavedLayouts,
			valid_text_file_name
		)?;
		db_manager.insert_layout(String::from(valid_text_file_name_excluding_postfix), &domain_board);
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
		let layout_name_string = format!("layout_{:?}", db_manager.saved_layouts.len());
		write_to_file(
			FolderToAccess::SavedLayouts,
			layout_name_string.clone(),
			layout_content_string
		).unwrap();

		db_manager.as_mut().insert_layout(layout_name_string, &save_request.0);
	}
}

fn listen_to_db_clearing_request(
	mut event_listener: EventReader<ClearDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	if let Err(system_access_error) =
		listen_to_db_clearing_request_inner(
			&mut event_listener,
			db_manager.as_mut()
		){
		print_system_access_error(system_access_error);
	}
}

fn listen_to_db_clearing_request_inner(
	event_listener: &mut EventReader<ClearDB>,
	db_manager: &mut DataBaseManager
) -> Result<(), SystemAccessError>
{
	for _clear_request in event_listener.read(){
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
				return Err(SystemAccessError::RequestedFileDoesntExist(FileName(valid_text_file_name)));
			}
			db_manager.remove_layout(String::from(valid_text_file_name_excluding_postfix));
		}
	}
	Ok(())
}

impl DataBaseManager{
	pub fn insert_layout(&mut self, name: String, layout: &DomainBoard){
		self.saved_layouts.insert(DomainBoardNameWithoutPostfix(name), layout.clone());
	}

	pub fn remove_layout(&mut self, name: String){
		self.saved_layouts.remove(&DomainBoardNameWithoutPostfix(name));
	}

	pub fn get_saved_layouts_ref(&self) -> &HashMap<DomainBoardNameWithoutPostfix, DomainBoard>{
		&self.saved_layouts
	}
}