use crate::prelude::*;
use enum_iterator::{all, Sequence};

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug, Hash, Default, Sequence)]
pub enum TileType {
    #[default]
    Empty,
    Numbered,
    Wall
}

impl TileType{
    pub fn to_atlas_index(&self) -> usize{
        match self{
            TileType::Empty => 0,
            TileType::Numbered => 1,
            TileType::Wall => 2,
        }
    }

    pub fn get_tile_types_as_vec() -> Vec<Self>{
        all::<Self>().collect::<Vec<_>>()
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