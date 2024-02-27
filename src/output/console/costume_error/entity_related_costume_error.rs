use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityRelatedCostumeError {
    NoEntity,
    EntityNotInQuery,
    DataStructError(DataStructError<Tile>),
}

pub fn print_entity_related_error(entity_error: EntityRelatedCostumeError) {
    error!("{:?}", entity_error);
}