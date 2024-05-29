use crate::output::game_session_log::append_to_game_session_log_file;
use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityRelatedCostumeError {
    NoEntity,
    EntityNotInQuery,
    DataStructError(DataStructError<Tile>),
}

pub fn print_entity_related_error(entity_error: EntityRelatedCostumeError) {
    let error_string = format!("{:?}", entity_error);
    append_to_game_session_log_file(error_string.clone());
    error!(error_string);
}