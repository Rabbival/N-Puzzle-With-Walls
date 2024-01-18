use crate::{costume_event::ui_event, prelude::*};

#[derive(Debug, Clone, Copy)]
pub enum GridError {
    InvalidIndex(GridLocation)
}

#[derive(Debug, Clone, Copy)]
pub enum GridTreeError {
    ParentNotFound,
    NodeAlreadyExists,
    NodeNotConnectedToTree,
    NodeNotFound
}

#[derive(Debug)]
pub enum MenuError {
    CantGoBeyondTileCountBounds(WallTilesChange),
}

#[derive(Debug, Clone, Copy)]
pub enum BoardGenerationError {
    VectorPermutationGenerationFailed,
    DirectionCouldntBeFlipped,
    ItemNotInMap(ItemNotFoundInMapError),
    TileMoveError(TileMoveError),
    CouldntPlaceAllWalls,
    NotEnoughAvailableSpots,
    GridTreeError(GridTreeError),
    GridError(GridError)
}

#[derive(Debug, Clone, Copy)]
pub enum ItemNotFoundInMapError {
    DirectionNotFoundInMap,
    EntityNotFoundInMap,
}

#[derive(Debug, Clone, Copy)]
pub enum TileMoveError {
    NoTileInCell(GridLocation),
    BoardFrozenToPlayer,
    NoEmptyNeighbor,
    PressedEmptySlot,
    NoOccupiedTileInThatDirection(BasicDirection),
    EntityRelated(EntityRelatedCustomError),
    TriedToSwitchWithAWall,
    TriedToSwitchEmptyWithEmpty,
    TriedToSwitchBetweenTwoOccupied(Tile, Tile),
    GridError(GridError)
}

#[derive(Debug, Clone, Copy)]
pub enum EntityRelatedCustomError {
    NoEntity,
    EntityNotInQuery,
    ItemNotInMap(ItemNotFoundInMapError),
}

pub struct ErrorHandlerPlugin;

impl Plugin for ErrorHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, board_generation_error_handler);
    }
}

pub fn board_generation_error_handler(
    mut event_listener: EventReader<ui_event::ShowGenerationError>,
) {
    for generation_error in event_listener.read() {
        print_board_generation_error(generation_error.0);
    }
}


/// I don't use it automatically inside the get set etc functions
/// since it they might have nothing to do with moving tiles
pub fn wrap_if_error<T>(result: Result<T, error_handler::GridError>) 
-> Result<T, error_handler::TileMoveError>{
    match result {
        Err(grid_error) => {
            Err(error_handler::TileMoveError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}
