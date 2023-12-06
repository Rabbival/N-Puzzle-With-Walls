use crate::logic::basic_direction;

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
    NoOccupiedTileInThatDirection (basic_direction::BasicDirection),
    EntityRelated(EntityRelatedCustomError)
}

#[derive(Debug)]
pub enum EntityRelatedCustomError{
    NoEntity,
    EntityNotInQuery,
    ItemNotInMap(ItemNotFoundInMapError)
}
