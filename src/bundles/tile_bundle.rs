use crate::prelude::*;

#[derive(Bundle)]
pub struct TileBundle{
    pub tile_type: TileType,
    pub tag: OnScreenTag
}

