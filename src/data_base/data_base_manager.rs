use std::fs;
use crate::costume_event::db_event;
use crate::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use bevy::a11y::accesskit::Role::Directory;

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
		//temp body
		let saved_layouts_ref = db_manager.get_saved_layouts_ref();
		
	}
}

fn save_to_data_base(
	event_listener: EventReader<db_event::SaveToDB>,
	db_manager: ResMut<DataBaseManager>
){
	save_to_data_base_inner(event_listener, db_manager).unwrap();
}

fn save_to_data_base_inner(
	mut event_listener: EventReader<db_event::SaveToDB>,
	mut db_manager: ResMut<DataBaseManager>
) -> std::io::Result<()>
{
	for save_request in event_listener.read(){
		db_manager.as_mut().insert_layout(&save_request.0);

		//TODO: a folder can't be created twice, create it only if it doesn't exist
		fs::create_dir("saved_layouts")?;
		let mut file = File::create("saved_layouts/file_name_lmao.txt")?;
		file.write_all(&format!("{:?}", save_request.0).as_bytes())?;
	}
	Ok(())
}

impl DataBaseManager{
	pub fn insert_layout(&mut self, layout: &DomainBoard){
		self.saved_layouts.push(layout.clone());
	}

	pub fn get_saved_layouts_ref(&self) -> &Vec<DomainBoard>{
		&self.saved_layouts
	}
}