use crate::prelude::*;

#[derive(Bundle)]
pub struct TileBundle{
    pub tile: Tile,
    pub tag: OnScreenTag
}