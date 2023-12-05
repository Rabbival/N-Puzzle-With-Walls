#[derive(Debug)]
pub enum TileMoveError{
    BoardFrozenToPlayer (String),
    IndexOutOfGridBounds (String),
    NoEmptyNeighbor (String),
    PressedEmptySlot (String),
    NoEntity,
    EntityNotInQuery
}

#[derive(Debug)]
pub enum BoardGenerationError{
    DirectionCouldntBeFlipped,
    DirectionNotFoundInMap,
}