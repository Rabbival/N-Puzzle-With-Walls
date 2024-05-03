use crate::prelude::*;

pub mod domain_board;
pub mod data_base_startup;
pub mod saved_layout_index;
pub mod domain_board_name;
pub mod data_base_direct_functions;
pub mod data_base_request_handler;

pub struct DataBasePlugin;

impl Plugin for DataBasePlugin{
	fn build(&self, app: &mut App) {
		app.init_resource::<DataBaseManager>()
			.add_plugins((
				DataBaseStartupPlugin,
				DataBaseRequestHandlerPlugin
			));
	}
}

pub const MAX_SAVED_LAYOUTS : u8 = 255;

#[derive(Resource, Default)]
pub struct DataBaseManager{
	saved_layouts: Vec<Entity>,
}

fn wrap_to_data_base_error<T>(result: Result<T, GridError>) -> Result<T, DataBaseError> {
	match result {
		Err(grid_error) => {
			Err(DataBaseError::CouldntBuildTileBoardFromWallLocations
				(BoardGenerationError::GridError(grid_error)))
		},
		Ok(value) => Ok(value)
	}
}