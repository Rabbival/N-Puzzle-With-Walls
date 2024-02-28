use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum TileMoveError {
    TileBoardError(TileBoardError),
    BoardFrozenToPlayer,
    NoEmptyNeighbor,
    PressedEmptySlot,
    NoOccupiedTileInThatDirection(BasicDirection),
    EntityRelated(EntityRelatedCostumeError),
    TriedToSwitchWithAWall,
    TriedToSwitchEmptyWithEmpty,
    TriedToSwitchBetweenTwoOccupied(Tile, Tile),
    GridError(GridError)
}

pub fn print_tile_move_error(move_error: TileMoveError) {
    match move_error {
        TileMoveError::BoardFrozenToPlayer => {
            warn!("board locked");
        }
        TileMoveError::NoEmptyNeighbor => {
            warn!("no empty neighbor");
        }
        TileMoveError::PressedEmptySlot => {
            warn!("pressed an empty slot");
        }
        TileMoveError::NoOccupiedTileInThatDirection(direction) => {
            warn!("no occupied tile in direction: {:?}", direction);
        }
        TileMoveError::TriedToSwitchEmptyWithEmpty => {
            info!("tried to switch empty with empty, hence no swap was performed");
        },
        TileMoveError::GridError(grid_error) => {
            match grid_error{
                GridError::InvalidPositionVector(position) => {
                    warn!("clicked position {:?} can't be converted to a valid grid location", position)
                },
                _ => error!("{:?}", grid_error)
            }
        },
        _ => {
            error!("{:?}", move_error)
        }
    }
}