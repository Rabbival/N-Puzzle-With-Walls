use crate::prelude::*;
use std::fmt;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Tile");
        debug_struct.field("t", &self.tile_type);
        if self.tile_type == TileType::Numbered{
            debug_struct
            .field("n", &(self.index+1));
        }else{
            debug_struct
            .field("i", &self.index);
        }
        debug_struct.finish()
    }
}