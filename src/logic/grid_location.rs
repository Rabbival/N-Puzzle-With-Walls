use crate::prelude::*;

#[derive(Component, Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct GridLocation{
    pub row: i32,
    pub col: i32
}

impl GridLocation {
    pub fn new(row: i32, col: i32) -> Self {
        GridLocation{
            row,
            col
        }
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let location = GridLocation{ 
            row: (-1.0*position.y/(ATLAS_CELL_SQUARE_SIZE)+0.5) as i32, 
            col: (position.x/(ATLAS_CELL_SQUARE_SIZE)+0.5) as i32
        };
        if Board::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }

    pub fn to_world(&self) -> Vec2{
        Vec2::new(
            (self.col as f32-0.5)*ATLAS_CELL_SQUARE_SIZE , 
            -1.0 * (self.row as f32-0.5)*ATLAS_CELL_SQUARE_SIZE
        )
    }
}

impl From<IVec2> for GridLocation {
    fn from(value: IVec2) -> Self {
        GridLocation{
            row: value.y,
            col: value.x
        }
    }
}