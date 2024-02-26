use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum EntityRelatedCostumeError {
    NoEntity,
    EntityNotInQuery,
    DataStructError(DataStructError<Tile>),
}
