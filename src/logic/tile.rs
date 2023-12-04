use crate::prelude::*;

#[derive(Component, Clone, Copy, Default)]
pub enum TileType {
    #[default]
    Empty,
    Numbered(u32),
}

#[derive(Component, Clone, Copy, Default)]
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
}