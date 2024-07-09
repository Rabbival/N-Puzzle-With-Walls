use crate::data_base::example_boards::ExampleBoardsPlugin;
use crate::prelude::*;

pub mod domain_board;
pub mod data_base_startup;
pub mod saved_layout_index;
pub mod domain_board_name;
pub mod data_base_direct_functions;
pub mod data_base_request_handler;
pub mod newborn_domain_board_name;
pub mod example_boards;

pub struct DataBasePlugin;

impl Plugin for DataBasePlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_plugins((
				DataBaseStartupPlugin,
				DataBaseRequestHandlerPlugin,
				NewbornDomainBoardNamePlugin,
				ExampleBoardsPlugin
			));
	}
}

pub const MAX_SAVED_LAYOUTS : u8 = 255;

#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: HashMap<BoardDifficulty, Vec<Entity>>,
}