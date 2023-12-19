use crate::prelude::*;

#[derive(Component, Clone, Copy, Default, PartialEq, Eq, Debug, Hash)]
pub enum TileType {
    #[default]
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

    pub fn to_atlas_index(&self) -> usize{
        match self{
            TileType::Empty => 0,
            TileType::Numbered(_) => 1,
        }
    }

    pub fn to_number(&self) -> Option<usize>{
        match self{
            TileType::Empty => None,
            TileType::Numbered(num) => Some(*num as usize),
        }
    }

    pub fn to_number_forced(&self, empty_value: usize) -> usize {
        match self{
            TileType::Empty => empty_value,
            TileType::Numbered(num) => *num as usize
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