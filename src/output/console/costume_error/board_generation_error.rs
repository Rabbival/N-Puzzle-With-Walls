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