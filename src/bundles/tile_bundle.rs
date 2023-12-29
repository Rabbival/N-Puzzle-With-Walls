use crate::prelude::*;

#[derive(Bundle)]
pub struct TileBundle{
    pub indexed_tile_type: IndexedValue<TileType>,
    pub tag: OnScreenTag
}

