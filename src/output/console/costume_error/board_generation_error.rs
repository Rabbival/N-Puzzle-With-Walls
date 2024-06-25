use crate::output::game_session_log::append_to_game_session_log_file;
use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum BoardGenerationError {
    VectorPermutationGenerationFailed,
    CouldntFindADirectionToMoveEmptyTileIn,
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
    let error_string = format!("board generation failed! error: {:?}", error);
    append_to_game_session_log_file(error_string.clone());
    error!(error_string);
}

impl From<GridError> for BoardGenerationError{
    fn from(value: GridError) -> Self {
        Self::GridError(value)
    }
}