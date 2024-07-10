use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileBoardError {
    NoTileInCell(GridLocation),
    GridError(GridError)
}

pub fn print_tile_board_error(board_error: TileBoardError) {
    match board_error {
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