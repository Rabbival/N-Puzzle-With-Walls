use std::ops::{Index, IndexMut};

use bevy::prelude::*;

use crate::prelude::*;

/// make sure to have a default you can tell apart
#[derive(Component, Clone, Debug)]
pub struct Grid<T: Default> {
    grid_side_length: u8,
    grid: HashMap<GridLocation, T>,
    default_container: T
}

//basics
impl<T: Default> Grid<T> {
    pub fn new(grid_side_length: u8) -> Self {
        Self {
            grid_side_length: grid_side_length,
            grid: HashMap::<GridLocation, T>::new(),
            default_container: T::default()
        }
    }

    ///immutable reference
    pub fn get_side_length(&self) -> &u8 {
        &self.grid_side_length
    }

    pub fn get(&self, location: &GridLocation) -> Option<&T> {
        self.grid.get(location)
    }

    pub fn get_mut(&self, location: &GridLocation) -> Option<&mut T> {
        self.grid.get_mut(location)
    }

    /// returns whether insertion was successful
    pub fn set(&self, location: &GridLocation, value: T) -> bool {
        if self.valid_index(location){
            self.grid.insert(location.clone(), value);
        }
        false
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&self, location: &GridLocation, value: T)-> Option<T>{
        self.grid.insert(location.clone(), value)
    }

    /// also returns false if the location is invalid, so remember to check that if relevant
    pub fn occupied(&self, location: &GridLocation) -> bool {
        if self.valid_index(location){
            return self.grid.contains_key(location);
        }
        false
    }

    /// object function in case we'd want to have grids of different sizes
    pub fn valid_index(&self, location: &GridLocation) -> bool {
        location.col >= 0
            && location.row >= 0
            && location.col < self.grid_side_length as i32
            && location.row < self.grid_side_length as i32
    }
}

impl<T: Default> Grid<T>{
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
}

//iterators
impl<T: Default> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (&GridLocation, &T)> + '_ {
        self.grid.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&GridLocation, &mut T)> + '_ {
        self.grid.iter_mut()
    }
}

impl<T: Default + PartialEq + Eq> PartialEq for Grid<T>{
    fn eq(&self, other: &Self) -> bool {
        let mut all_cells_are_equal=true;
        let self_iter=self.iter();
        let other_iter=other.iter();
        for ((_ , self_value), ( _ , other_value)) in self_iter.zip(other_iter) {
            if self_value != other_value{
                all_cells_are_equal=false;
                break;
            }
        }
        all_cells_are_equal
    }
}
impl<T: Default + PartialEq + Eq> Eq for Grid<T>{}

/// in case there's no value with this location, 
/// a reference to the default container will be provided instead
impl<T: Default> Index<&GridLocation> for Grid<T> {
    type Output = T;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        if let Some(value) = self.get(index){
            return value;
        }
        &self.default_container
    }
}

/// in case there's no value with this location, 
/// a reference to the default container will be provided instead
impl<T: Default> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        if let Some(value) = self.get_mut(index){
            return value;
        }
        &mut self.default_container
    }
}