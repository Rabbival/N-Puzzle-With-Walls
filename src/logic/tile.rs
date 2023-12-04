use crate::prelude::*;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq)]
pub enum TileType {
    #[default]
    Empty,
    Numbered(u32),
}

#[derive(Component, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tile{
    pub tile_type: TileType,
}

impl Tile{
    pub fn new(possibly_number: Option<u32>) -> Self{
        Self { tile_type: match possibly_number{
            None=>TileType::Empty,
            Some(number)=>TileType::Numbered(number)
        } }
    }

    pub fn to_index(&self) -> usize{
        match self.tile_type{
            TileType::Empty => 15,
            TileType::Numbered(num) => num as usize,
        }
    }
}