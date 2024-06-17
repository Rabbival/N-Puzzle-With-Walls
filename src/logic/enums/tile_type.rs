use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, Sequence, Deserialize, Serialize)]
pub enum TileType {
    #[default]
    Empty,
    Numbered,
    Wall,
}

impl TileType {
    pub fn to_tiles_atlas_index(&self) -> usize {
        match self {
            TileType::Empty => 0,
            TileType::Numbered => 1,
            TileType::Wall => 2,
        }
    }
}

impl PartialEq<&TileType> for TileType {
    fn eq(&self, other: &&Self) -> bool {
        self == *other
    }
}
impl PartialEq<TileType> for &TileType {
    fn eq(&self, other: &TileType) -> bool {
        *self == other
    }
}
