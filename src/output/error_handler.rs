#[derive(Debug)]
pub enum BoardGenerationError{
    DirectionCouldntBeFlipped,
    DirectionNotFoundInMap,
}

#[derive(Debug)]
pub enum TileMoveError{
    BoardFrozenToPlayer (String),
    IndexOutOfGridBounds (String),
    NoEmptyNeighbor (String),
    PressedEmptySlot (String),
    EntityRelated(EntityRelatedCustomError)
}

#[derive(Debug)]
pub enum EntityRelatedCustomError{
    NoEntity,
    EntityNotInQuery
}