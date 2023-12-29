use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Tile{
    pub index: usize,
    pub tile_type: TileType
}

impl Tile{
    /// indexes with 0
    pub fn new(tile_type: TileType)-> Self{
        Self { index: 0, tile_type }
    }
}