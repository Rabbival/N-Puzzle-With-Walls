
use crate::costume_event::db_event;
use crate::input::ron_loader;
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
			.add_systems(Update, (
				draw_from_data_base,
				save_to_data_base
			));
	}
}

fn draw_from_data_base(
	mut event_listener: EventReader<db_event::LoadFromDB>,
	db_manager: Res<DataBaseManager>
){
	for load_request in event_listener.read(){
		let requested_layout_index = load_request.0.0;
		let saved_layouts_ref = db_manager.get_saved_layouts_ref();
		if requested_layout_index >= saved_layouts_ref.len(){
			print_to_console::print_system_log(SystemLog::RequestedFileDoesntExist);
		}else{
			let parsed_ron = ron_loader::domain_board_from_file(
				FolderToAccess::SavedLayouts,
				format!("layout_{:?}",requested_layout_index)
			);
			if parsed_ron.is_err(){
				print_to_console::print_system_log(SystemLog::RequestedFileDoesntExist);
			}else{
				let domain_board = parsed_ron.unwrap();
			}
		}
	}
}

fn save_to_data_base(
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