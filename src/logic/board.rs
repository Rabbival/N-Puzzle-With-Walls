use std::{ops::{Index,IndexMut}, sync::{Arc, RwLock}};

use bevy::prelude::*;

use crate::prelude::*;

pub const GRID_SIZE: u32 = 4;

#[derive(Component, Clone, Debug)]
pub struct Board<T> {
    pub grid: [[Arc::<RwLock::<Option<T>>>; GRID_SIZE as usize]; GRID_SIZE as usize],
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
}

//basics
impl<T> Board<T> {
    fn new() -> Self {
        let grid = std::array::from_fn(|_| std::array::from_fn(|_| {
            Arc::new(RwLock::new(None))
        }));
        Self {
            grid,
            empty_tile_location: GridLocation::default(),
            ignore_player_input: true
        }
    }
}

impl<T: PartialEq + Eq> PartialEq for Board<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut all_cells_are_equal=true;
        for row_index in 0..GRID_SIZE{
            for col_index in 0..GRID_SIZE{
                let location=GridLocation::new(row_index as i32, col_index as i32);
                if self[&location] != other[&location]{
                    all_cells_are_equal=false;
                    break;
                }
            }
        }
        all_cells_are_equal
    }
}
impl<T: PartialEq + Eq> Eq for Board<T>{}

impl<T: PartialEq + Eq> Index<&GridLocation> for Board<T> {
    type Output = Option<T>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        let cell_value = &self.grid[index.row as usize][index.col as usize];
        &Arc::clone(&cell_value).into_inner().unwrap()
    }
}

impl<T: PartialEq + Eq> IndexMut<&GridLocation> for Board<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        let cell_value = &mut self.grid[index.row as usize][index.col as usize];
        &mut Arc::clone(&cell_value).into_inner().unwrap()
    }
}