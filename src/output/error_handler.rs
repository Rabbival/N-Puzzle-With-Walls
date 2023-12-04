#[derive(Debug)]
pub enum InputHandlerError{
    IndexOutOfGridBounds (String),
    GridLocationOccupied (String),
}

#[derive(Debug)]
pub enum BoardGenerationError{
    DirectionCouldntBeFlipped,
    DirectionNotFoundInMap,
    BoardAlreadySolved
}