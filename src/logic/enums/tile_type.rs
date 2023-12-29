use crate::prelude::*;

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TileType {
    Empty(u32),
    Numbered(u32),
    Wall(u32)
}

impl TileType{
    pub fn to_atlas_index(&self) -> usize{
        match self{
            TileType::Empty(_) => 0,
            TileType::Numbered(_) => 1,
            TileType::Wall(_) => 2,
        }
    }

    pub fn to_tile_index(&self) -> u32{
        match self{
            TileType::Empty(index) | TileType::Wall(index) => *index,
            TileType::Numbered(num) => *num,
        }
    }
}

// enable comparison to &TileType from both sides
impl PartialEq<&TileType> for TileType{
    fn eq(&self, other: &&Self) -> bool {
        self==*other
    }
}
impl PartialEq<TileType> for &TileType{
    fn eq(&self, other: &TileType) -> bool {
        *self==other
    }
}

impl Default for TileType{
    fn default() -> Self {
        Self::Empty(0)
    }
}