use crate::prelude::*;

#[derive(Component, Default, Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct GridLocation{
    pub row: i32,
    pub col: i32
}

impl GridLocation {
    pub fn new(row: i32, col: i32) -> Self {
        Self{
            row,
            col
        }
    }

    pub fn from_index(index: u8, grid_side_length: u8) -> Self{
        Self{
            row: (index / grid_side_length) as i32,
            col: (index % grid_side_length) as i32
        }
    }

    pub fn to_index(&self, grid_side_length: u8) -> usize{
        ( self.row * grid_side_length as i32 + self.col ) as usize
    }

    /// grid provided to check that the index is valid for its size
    pub fn from_world<T: Clone>(grid: &Grid<T>, position: Vec2) -> Option<Self> {
        let location = GridLocation{ 
            row: (-1.0*position.y/(ATLAS_CELL_SQUARE_SIZE)+0.5) as i32, 
            col: (position.x/(ATLAS_CELL_SQUARE_SIZE)+0.5) as i32
        };
        if grid.valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }

    pub fn to_world(&self) -> Vec3{
        Vec3::new(
            (self.col as f32)*ATLAS_CELL_SQUARE_SIZE , 
            -1.0 * (self.row as f32)*ATLAS_CELL_SQUARE_SIZE,
            0.0
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