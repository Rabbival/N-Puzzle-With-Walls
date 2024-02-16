use crate::{costume_event::ui_event, prelude::*};

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
    DirectionNotInMap(DataStructError<BasicDirection>),
    CircleCheckError(DataStructError<GridLocation>),
    TileMoveError(TileMoveError),
    CouldntPlaceAllWalls,
    NotEnoughAvailableSpots,
    GridTreeError(GridTreeError),
    GridError(GridError)
}

#[derive(Debug, Clone, Copy)]
pub enum DataStructError<T> {
    ItemNotFound(T),
    KeyAlreadyExists,
    GridTreeError(GridTreeError)
}

#[derive(Debug, Clone, Copy)]
pub enum TileMoveError {
    TileBoardError(TileBoardError),
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
pub enum TileBoardError {
    NoTileInCell(GridLocation),
    GridError(GridError)
}

#[derive(Debug, Clone, Copy)]
pub enum GridError {
    InvalidIndex(GridLocation),
    InvalidPositionVector(Vec2),
}

#[derive(Debug, Clone, Copy)]
pub enum EntityRelatedCustomError {
    NoEntity,
    EntityNotInQuery,
    DataStructError(DataStructError<Tile>),
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