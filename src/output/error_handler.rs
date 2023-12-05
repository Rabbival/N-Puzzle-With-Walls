#[derive(Debug)]
pub enum BoardGenerationError{
    DirectionCouldntBeFlipped,
    ItemNotInMap(ItemNotFoundInMapError)
}

#[derive(Debug)]
pub enum ItemNotFoundInMapError{
    DirectionNotFoundInMap,
    EntityNotFoundInMap
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
    EntityNotInQuery,
    ItemNotInMap(ItemNotFoundInMapError)
}
