use std::fs;
use crate::costume_event::db_event;
use crate::system::ron_loader;
use crate::output::{print_to_console, text_saver};
use crate::prelude::*;

#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: Vec<DomainBoard>,
}

pub struct DataBaseManagerPlugin;

impl Plugin for DataBaseManagerPlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_systems(Startup, read_system_files_into_db)
			.add_systems(Update, save_to_data_base_and_system);
	}
}

// TODO: handle the errors more gracefully
fn read_system_files_into_db(
	mut db_manager: ResMut<DataBaseManager>
){
	let saved_layouts_directory_iterator = fs::read_dir(FolderToAccess::SavedLayouts.to_string()).unwrap();
	for layout_file_result in saved_layouts_directory_iterator{
		if layout_file_result.is_ok(){
			let layout_file_name = layout_file_result.unwrap().file_name().into_string().unwrap();
			if &layout_file_name[(layout_file_name.len()-4)..layout_file_name.len()] != ".txt"{
				//TODO: error throw here "not a text file"
				panic!()
			}

			info!("{:?}", layout_file_name);


			let parsed_ron = ron_loader::domain_board_from_file(
				FolderToAccess::SavedLayouts,
				layout_file_name
			);
			if parsed_ron.is_err(){
				print_to_console::print_system_log(SystemLog::RequestedFileDoesntExist);
			}else{
				let domain_board = parsed_ron.unwrap();
				db_manager.insert_layout(&domain_board);


				info!("{:?}", db_manager.saved_layouts);


			}
		}
	}
}

fn save_to_data_base_and_system(
	mut event_listener: EventReader<db_event::SaveToDB>,
	mut db_manager: ResMut<DataBaseManager>
){
	for save_request in event_listener.read(){
		let layout_ron_string = ron::ser::to_string_pretty(
			&save_request.0, ron::ser::PrettyConfig::default()).unwrap();
		text_saver::write_to_file(
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