use crate::prelude::TileType;

#[derive(Debug)]
pub enum InputHandlerError{
    BoardLocked (String),
    IndexOutOfGridBounds (String),
    NoEmptyNeighbor (String),
    PressedEmptySlot (String)
}

#[derive(Debug)]
pub enum BoardGenerationError{
    DirectionCouldntBeFlipped,
    DirectionNotFoundInMap,
}

#[derive(Debug)]
pub enum SearchError{
    TileNotFound (TileType),
    TilesNotFound (TileType, TileType),
}