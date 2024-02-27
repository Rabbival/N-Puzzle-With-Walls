use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum BoardGenerationError {
    VectorPermutationGenerationFailed,
    DirectionCouldntBeFlipped,
    DirectionNotInMap(DataStructError<BasicDirection>),
    CircleCheckError(DataStructError<GridLocation>),
    TileMoveError(TileMoveError),
    CouldntPlaceAllWalls,
    NotEnoughAvailableSpots,
    GridTreeError(GridTreeError),
    GridError(GridError)
}

pub fn print_board_generation_error(error: BoardGenerationError) {
    error!("board generation failed! error: {:?}", error);
}