use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Tile{
    pub index: usize,
    pub value: TileType
}

impl Tile{
    /// indexes with 0
    pub fn new(value: TileType)-> Self{
        Self { index: 0, value }
    }
}