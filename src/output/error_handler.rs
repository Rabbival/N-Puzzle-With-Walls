use crate::{prelude::*, costume_event::ui_event};

#[derive(Debug)]
pub enum MenuError{
    CantGoBeyondTileCountBounds(WallTilesChange)
}

#[derive(Debug, Clone, Copy)]
pub enum BoardGenerationError{
    VectorPermutationGenerationFailed,
    DirectionCouldntBeFlipped,
    ItemNotInMap(ItemNotFoundInMapError),
    TileMoveError(TileMoveError),
    CouldntPlaceAllWalls,
    NotEnoughAvailableSpots
}

#[derive(Debug, Clone, Copy)]
pub enum ItemNotFoundInMapError{
    DirectionNotFoundInMap,
    EntityNotFoundInMap 
}

#[derive(Debug, Clone, Copy)]
pub enum TileMoveError{
    NoTileInCell(GridLocation),
    BoardFrozenToPlayer,
    IndexOutOfGridBounds,
    NoEmptyNeighbor,
    PressedEmptySlot,
    NoOccupiedTileInThatDirection (BasicDirection),
    EntityRelated(EntityRelatedCustomError),
    TriedToSwitchWithAWall,
    TriedToSwitchEmptyWithEmpty,
    TriedToSwitchBetweenTwoOccupied(Tile,Tile)
}

#[derive(Debug, Clone, Copy)]
pub enum EntityRelatedCustomError{
    NoEntity,
    EntityNotInQuery,
    ItemNotInMap(ItemNotFoundInMapError)
}


pub struct ErrorHandlerPlugin;

impl Plugin for ErrorHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, board_generation_error_handler)
        ;
    }
}


pub fn board_generation_error_handler(
    mut event_listener: EventReader<ui_event::ShowGenerationError>,
){
    for generation_error in event_listener.read(){
        print_board_generation_error(generation_error.0);
    }
}