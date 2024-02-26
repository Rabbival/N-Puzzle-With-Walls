use crate::prelude::*;

pub mod grid_tree_error;
pub mod tile_board_error;
pub mod grid_error;

#[derive(Debug, Clone, Copy)]
pub enum DataStructError<T> {
    ItemNotFound(T),
    KeyAlreadyExists,
    GridTreeError(GridTreeError)
}
