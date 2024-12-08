use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TileInDirectLine {
    pub tile: Tile,
    pub direction: BasicDirection,
    pub distance: i32,
}

impl TileInDirectLine {
    pub fn from_origin_and_location(
        origin: &GridLocation,
        location: &GridLocation,
        location_tile: &Tile,
    ) -> TileInDirectLine {
        let tile = *location_tile;
        if location.col > origin.col {
            TileInDirectLine {
                tile,
                direction: BasicDirection::Right,
                distance: location.col - origin.col,
            }
        } else if location.col < origin.col {
            TileInDirectLine {
                tile,
                direction: BasicDirection::Left,
                distance: origin.col - location.col,
            }
        } else if location.row > origin.row {
            TileInDirectLine {
                tile,
                direction: BasicDirection::Down,
                distance: location.row - origin.row,
            }
        } else {
            TileInDirectLine {
                tile,
                direction: BasicDirection::Up,
                distance: origin.col - location.col,
            }
        }
    }

    pub fn try_to_tiles_shift_request(&self) -> Option<ShiftTilesInDirectionRequest> {
        if self.distance < 0 {
            None
        } else {
            if let Some(opposite_direction) = self.direction.opposite_direction() {
                Some(ShiftTilesInDirectionRequest {
                    move_neighbor_from_direction: opposite_direction,
                    empty_tile_index: self.tile.index,
                    steps_count: self.distance as usize,
                })
            } else {
                None
            }
        }
    }
}
