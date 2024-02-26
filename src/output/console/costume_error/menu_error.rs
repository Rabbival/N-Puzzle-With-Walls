use crate::prelude::*;

#[derive(Debug)]
pub enum MenuError {
    CantGoBeyondTileCountBounds(WallTilesChange),
}