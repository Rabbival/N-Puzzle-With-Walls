use crate::prelude::*;

pub mod domain_board;
pub mod data_base_manager;
pub mod domain_board_index;

pub struct DataBasePlugins;

impl Plugin for DataBasePlugins{
	fn build(&self, app: &mut App) {
		app.add_plugins(DataBaseManagerPlugin);
	}
}