use crate::logic::{enums::basic_direction, grid};

#[derive(Debug)]
pub enum BoardGenerationError{
    DirectionCouldntBeFlipped,
    ItemNotInMap(ItemNotFoundInMapError),
    TileMoveError
}

#[derive(Debug)]
pub enum ItemNotFoundInMapError{
    DirectionNotFoundInMap,
    EntityNotFoundInMap 
}

#[derive(Debug)]
pub enum TileMoveError{
    NoTileInCell(grid::grid_location::GridLocation),
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
