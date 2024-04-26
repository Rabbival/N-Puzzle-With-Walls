use crate::prelude::{DomainBoardName, error, print_system_access_error, SystemAccessError};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum BoardLoadingError {
    WallListDoesntMatchWallCount(DomainBoardName),
    SystemAccessError(SystemAccessError)
}

pub fn print_board_loading_error(board_loading_error: BoardLoadingError) {
    match board_loading_error {
        BoardLoadingError::WallListDoesntMatchWallCount(board_name) => {
            error!(
                "the specified wall count doesn't match the amount of
                specified board locations for board {}, so it wasn't loaded",
                board_name
            );
        },
        BoardLoadingError::SystemAccessError(system_access_error) => {
            print_system_access_error(system_access_error);
        }
    }
}