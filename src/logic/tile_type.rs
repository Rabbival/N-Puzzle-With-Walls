use crate::prelude::*;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Debug, Hash)]
pub enum TileType {
    #[default]
    NoTileNorEmpty,
    Empty,
    Numbered(u32),
}

impl TileType{
    pub fn new(possibly_number: Option<u32>) -> Self{
        match possibly_number{
            None=>TileType::Empty,
            Some(number)=>TileType::Numbered(number)
        }
    }

    pub fn to_atlas_index(&self) -> Option<usize>{
        match self{
            TileType::NoTileNorEmpty => None,
            TileType::Empty => Some(0),
            TileType::Numbered(_) => Some(1),
        }
    }

    pub fn to_number(&self) -> Option<usize>{
        match self{
            TileType::NoTileNorEmpty => None,
            TileType::Empty => None,
            TileType::Numbered(num) => Some(*num as usize),
        }
    }
}