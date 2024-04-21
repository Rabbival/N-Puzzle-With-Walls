use crate::prelude::*;

pub mod domain_board;
pub mod data_base_manager;

pub struct DataBasePlugin;

impl Plugin for DataBasePlugin{
	fn build(&self, app: &mut App) {
		app.add_plugins(DataBaseManagerPlugin);
	}
}