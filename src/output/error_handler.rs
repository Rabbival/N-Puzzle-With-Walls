use crate::prelude::TileType;

#[derive(Debug)]
pub enum InputHandlerError{
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
pub enum InitializationError{
    NoTileTranslationConfigured (TileType)
}