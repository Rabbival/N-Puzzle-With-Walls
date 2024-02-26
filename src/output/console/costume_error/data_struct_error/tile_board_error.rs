use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileBoardError {
    NoTileInCell(GridLocation),
    GridError(GridError)
}