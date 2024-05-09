use crate::prelude::{BoardGenerationError, error, print_system_access_error, SystemAccessError};

#[derive(Debug, Clone)]
pub enum DataBaseError {
    SystemAccessError(SystemAccessError),
    CouldntBuildTileBoardFromWallLocations(BoardGenerationError)
}

pub fn print_data_base_error(data_base_error: DataBaseError) {
    match data_base_error {
        DataBaseError::SystemAccessError(system_access_error) => {
            print_system_access_error(system_access_error);
        },
        DataBaseError::CouldntBuildTileBoardFromWallLocations(board_building_error) => {
            error!("Couldn't build tile board from wall locations due to the following error: {:?}", board_building_error);
        }
    }
}