use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileBoardError {
    TileIndexOutOfBounds(usize),
    NoTileInCell(GridLocation),
    GridError(GridError)
}

pub fn print_tile_board_error(board_error: TileBoardError) {
    match board_error {
        TileBoardError::TileIndexOutOfBounds(requested_index) => {
            error!("tile index {:?} out of bounds", requested_index);
        }
        TileBoardError::NoTileInCell(location) => {
            error!("no tile in cell: {}", location);
        },
        tile_board_error => error!("{:?}", tile_board_error),
    }
}

impl From<GridError> for TileBoardError{
    fn from(value: GridError) -> Self {
        TileBoardError::GridError(value)
    }
}