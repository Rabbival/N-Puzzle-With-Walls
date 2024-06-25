use crate::prelude::*;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Default, Deserialize, Serialize)]
pub struct Tile {
    pub index: usize,
    pub tile_type: TileType,
}

impl Tile {
    /// indexes with 0
    pub fn new(tile_type: TileType) -> Self {
        Self {
            index: 0,
            tile_type,
        }
    }

    pub fn to_tiles_atlas_index(&self) -> usize{
        self.tile_type.to_tiles_atlas_index()
    }

    pub fn to_regular_arrows_atlas_index(&self) -> Option<usize>{
        if self.tile_type == TileType::Empty{
            Some(self.index * 2)
        }else{
            None
        }
    }

    pub fn to_highlighted_arrows_atlas_index(&self) -> Option<usize>{
        Some(self.to_regular_arrows_atlas_index()?+1)
    }
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("Tile");
        debug_struct.field("type", &self.tile_type);
        if self.tile_type == TileType::Numbered {
            debug_struct.field("number", &(self.index + 1));
        } else {
            debug_struct.field("index", &self.index);
        }
        debug_struct.finish()
    }
}
