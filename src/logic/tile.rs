use crate::prelude::*;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum TileType {
    #[default]
    Empty,
    Numbered(u32),
}

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct Tile{
    pub tile_type: TileType,
    pub tile_entity: Option<Entity>
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_type == other.tile_type
    }
}
impl Eq for Tile{}

impl Tile{
    pub fn new(possibly_number: Option<u32>) -> Self{
        Self { 
            tile_type: match possibly_number{
                None=>TileType::Empty,
                Some(number)=>TileType::Numbered(number)
            },
            tile_entity: None
        }
    }

    pub fn to_atlas_index(&self) -> usize{
        match self.tile_type{
            TileType::Empty => 15,
            TileType::Numbered(num) => num as usize -1,
        }
    }
}