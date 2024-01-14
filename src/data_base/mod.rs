use crate::prelude::*;

pub mod saved_layout;
pub mod data_base_manager;

pub struct DataBasePlugins;

impl Plugin for DataBasePlugins{
	fn build(&self, app: &mut App) {
		app.add_plugins(DataBaseManagerPlugin);
	}
}