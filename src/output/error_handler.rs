use crate::logic::{enums::{basic_direction, board_property_enums::wall_tiles_change}, data_structure::grid_related};

#[derive(Debug)]
pub enum MenuError{
    CantGoBeyondTileCountBounds(wall_tiles_change::WallTilesChange)
}

#[derive(Debug)]
pub enum BoardGenerationError{
    VectorPermutationGenerationFailed,
    DirectionCouldntBeFlipped,
    ItemNotInMap(ItemNotFoundInMapError),
    TileMoveError,
    GridError(GridError)
}

#[derive(Debug)]
pub enum ItemNotFoundInMapError{
    DirectionNotFoundInMap,
    EntityNotFoundInMap 
}

#[derive(Debug)]
pub enum TileMoveError{
    NoTileInCell(grid_related::grid_location::GridLocation),
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

#[derive(Debug)]
pub enum GridError{
    IteratorYieldedNone,
}
