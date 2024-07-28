use crate::output::game_session_log::append_to_game_session_log_file;
use crate::prelude::{DomainBoardName, error, print_system_access_error, SystemAccessError};

#[derive(Debug, Clone)]
pub enum DataBaseError {
    SystemAccessError(SystemAccessError),
    MismatchedGridAndProperties(DomainBoardName)
}

pub fn print_data_base_error(data_base_error: DataBaseError) {
    match data_base_error {
        DataBaseError::SystemAccessError(system_access_error) => {
            print_system_access_error(system_access_error);
        },
        DataBaseError::MismatchedGridAndProperties(board_name) => {
            let error_string =
                format!("The board's grid and properties don't match for {:?}", board_name);
            append_to_game_session_log_file(error_string.clone());
            error!(error_string);
        }
    }
}

impl From<SystemAccessError> for DataBaseError{
    fn from(value: SystemAccessError) -> Self {
        Self::SystemAccessError(value)
    }
}