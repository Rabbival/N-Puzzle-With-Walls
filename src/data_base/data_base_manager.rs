use std::fs;
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: Vec<DomainBoard>,
}

pub struct DataBaseManagerPlugin;

impl Plugin for DataBaseManagerPlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_systems(Startup, read_system_text_files_into_db)
			.add_systems(Update, save_to_data_base_and_system);
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
	let saved_layouts_directory_iterator
		= fs::read_dir(FolderToAccess::SavedLayouts.to_string()).unwrap();
	for layout_file_result in saved_layouts_directory_iterator{
		if layout_file_result.is_ok(){
			let layout_file_name = layout_file_result.unwrap().file_name().into_string().unwrap();
			let file_name_prefix = &layout_file_name[(layout_file_name.len()-4)..layout_file_name.len()];
			if file_name_prefix == ".txt"{
				let domain_board = domain_board_from_file(
					FolderToAccess::SavedLayouts,
					layout_file_name
				)?;
				db_manager.insert_layout(&domain_board);
			}
		}
	}
	Ok(())
}

fn save_to_data_base_and_system(
	mut event_listener: EventReader<SaveToDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	for save_request in event_listener.read(){
		let layout_ron_string = ron::ser::to_string_pretty(
			&save_request.0, ron::ser::PrettyConfig::default()).unwrap();
		write_to_file(
			FolderToAccess::SavedLayouts,
			format!("layout_{:?}", db_manager.saved_layouts.len()),
			layout_ron_string
		).unwrap();

		db_manager.as_mut().insert_layout(&save_request.0);
	}
}

impl DataBaseManager{
	pub fn insert_layout(&mut self, layout: &DomainBoard){
		self.saved_layouts.push(layout.clone());
	}

	pub fn get_saved_layouts_ref(&self) -> &Vec<DomainBoard>{
		&self.saved_layouts
	}
}