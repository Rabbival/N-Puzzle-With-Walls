use crate::{prelude::*, costume_event::app_event};

#[derive(Resource, Default)]
pub struct DataBaseManager{
	pub saved_layouts: Vec<SavedLayout>
}

pub struct DataBaseManagerPlugin;

impl Plugin for DataBaseManagerPlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_systems(Update, draw_from_data_base);
	}
}

fn draw_from_data_base(
	mut event_listener: EventReader<app_event::ToggleMenu>, //temp
	db_manager: Res<DataBaseManager>
){
	//eventually pass index to draw in the event
	for _event in event_listener.read(){
		//temp body
		let saved_layouts_ref = db_manager.get_saved_layouts_ref();
		
	}
}

impl DataBaseManager{
	pub fn insert_layout(&mut self, layout: SavedLayout){
		self.saved_layouts.push(layout);
	}

	pub fn get_saved_layouts_ref(&self) -> &Vec<SavedLayout>{
		&self.saved_layouts
	}
}