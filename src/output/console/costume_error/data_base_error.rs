use crate::prelude::{BoardGenerationError, DomainBoardName, error, print_system_access_error, SystemAccessError};

#[derive(Debug, Clone)]
pub enum DataBaseError {
    WallListDoesntMatchWallCount(DomainBoardName),
    SystemAccessError(SystemAccessError),
    CouldntBuildTileBoardFromWallLocations(BoardGenerationError)
}

pub fn print_data_base_error(data_base_error: DataBaseError) {
    match data_base_error {
        DataBaseError::WallListDoesntMatchWallCount(board_name) => {
            error!(
                "The specified wall count doesn't match the amount of
                specified board locations for board {}, so it wasn't loaded",
                board_name
            );
        },
        DataBaseError::SystemAccessError(system_access_error) => {
            print_system_access_error(system_access_error);
        },
        DataBaseError::CouldntBuildTileBoardFromWallLocations(board_building_error) => {
            error!("Couldn't build tile board from wall locations due to the following error: {:?}", board_building_error);
        }
    }
}