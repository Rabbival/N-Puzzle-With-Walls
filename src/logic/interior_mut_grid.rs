use std::{ops::{Index,IndexMut}, sync::{Arc, RwLock}};

use bevy::prelude::*;

use crate::prelude::*;

pub const GRID_SIZE: u32 = 4;

#[derive(Component, Clone, Debug)]
pub struct InteriorMutGrid<T> {
    pub grid: [[Arc::<RwLock::<Option<T>>>; GRID_SIZE as usize]; GRID_SIZE as usize],
}

impl<T> InteriorMutGrid<T>{
    /// only returns occupied ones 
    pub fn get_all_direct_neighbor_locations(&self, origin: &GridLocation) 
    -> HashMap<BasicDirection, GridLocation>
    {
        let mut valid_neighbors:HashMap<BasicDirection, GridLocation>=HashMap::new();
        for dir in BasicDirection::get_directions_as_vec(){
            if let Some(neighbor_location) = self.occupied_neighbor_location(origin, &dir){
                valid_neighbors.insert(dir,neighbor_location);
            }
        }
        valid_neighbors
    }

    fn occupied_neighbor_location(
            &self, 
            origin: &GridLocation, 
            dir: &BasicDirection
        ) -> Option<GridLocation>
        {
        let neighbor_location = self.neighbor_location(origin, dir);
        if self.occupied(&neighbor_location){
            Some(neighbor_location)
        }else{
            None
        }
    }

    //returns a location without checking if it's valid
    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> GridLocation{
        match dir{
            BasicDirection::Up=>GridLocation::new(origin.row-1, origin.col),
            BasicDirection::Right=>GridLocation::new(origin.row, origin.col+1),
            BasicDirection::Down=>GridLocation::new(origin.row+1, origin.col),
            BasicDirection::Left=>GridLocation::new(origin.row, origin.col-1)
        }
    }

    /// also returns false if the location is invalid, so remember to check that if relevant
    pub fn occupied(&self, location: &GridLocation) -> bool {
        if self.valid_index(location){
            match self[location]{
                None=> {return false;},
                Some(_)=> {return true;}
            }
        }
        false
    }

    /// object function in case we'd want to have grids of different sizes
    pub fn valid_index(&self, location: &GridLocation) -> bool {
        location.col >= 0
            && location.row >= 0
            && location.col < GRID_SIZE as i32
            && location.row < GRID_SIZE as i32
    }
}

impl<T> InteriorMutGrid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (GridLocation, Arc<RwLock<Option<T>>>)> + '_ {
        self.grid
            .iter()
            .flatten()
            .enumerate()
            .map(|(i, cell_value)| {
                (
                    GridLocation::new((
                        i as u32 / GRID_SIZE) as i32,
                        (i as u32 % GRID_SIZE) as i32
                    ),
                    Arc::clone(cell_value),
                )
            })
    }
}

impl<T> Default for InteriorMutGrid<T> {
    fn default() -> Self {
        Self {
            grid: std::array::from_fn(|_| std::array::from_fn(|_| {
                Arc::new(RwLock::new(None))
            }))
        }
    }
}

impl<T: PartialEq + Eq> PartialEq for InteriorMutGrid<T>{
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
impl<T: PartialEq + Eq> Eq for InteriorMutGrid<T>{}

impl<T> Index<&GridLocation> for InteriorMutGrid<T> {
    type Output = Option<T>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        let cell_value = &self.grid[index.row as usize][index.col as usize];
        &Arc::clone(&cell_value).into_inner().unwrap()
    }
}

impl<T> IndexMut<&GridLocation> for InteriorMutGrid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        let cell_value = &mut self.grid[index.row as usize][index.col as usize];
        &mut Arc::clone(&cell_value).into_inner().unwrap()
    }
}