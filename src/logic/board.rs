use std::{ops::{Index,IndexMut}, sync::{Arc, RwLock}};

use bevy::{prelude::*, utils::HashMap};

use crate::prelude::*;

pub const GRID_SIZE: u32 = 4;

#[derive(Component, Clone, Debug)]
pub struct Board<T> {
    pub grid: [[Option<Arc::<RwLock::<T>>>; GRID_SIZE as usize]; GRID_SIZE as usize],
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
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

//basics
impl<T> Board<T> {
    fn new() -> Self {
        let grid = std::array::from_fn(|_| std::array::from_fn(|_| None));
        Self {
            grid,
            empty_tile_location: GridLocation::default(),
            ignore_player_input: true
        }
    }
}

impl<Tile> Board<Tile> {
    // assumes one is empty
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation){
        if self[first].tile_type==TileType::Empty{
            self.empty_tile_location=second.clone();
        }else{
            self.empty_tile_location=first.clone();
        }

        let temp_tile=self[first];
        self[first]=self[second];
        self[second]=temp_tile;
    }

    pub fn get_direct_neighbors_of_empty(&self) -> HashMap<BasicDirection, GridLocation>{
        self.get_all_direct_neighbor_locations(&self.empty_tile_location) 
    }

    pub fn get_empty_neighbor(&self, origin: &GridLocation) -> Option<GridLocation>{
        for dir in BasicDirection::get_directions_as_vec(){
            let neighbor_location = self.neighbor_location(origin, &dir);
            if Board::<Tile>::valid_index(&neighbor_location) && !self.occupied(&neighbor_location){
                return Some(neighbor_location);
            }
        }
        None
    }

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

    pub fn occupied_neighbor_location(
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

    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> GridLocation{
        match dir{
            BasicDirection::Up=>GridLocation::new(origin.row-1, origin.col),
            BasicDirection::Right=>GridLocation::new(origin.row, origin.col+1),
            BasicDirection::Down=>GridLocation::new(origin.row+1, origin.col),
            BasicDirection::Left=>GridLocation::new(origin.row, origin.col-1)
        }
    }

    pub fn occupied(&self, location: &GridLocation) -> bool {
        if Board::<Tile>::valid_index(location){
            match self[location].tile_type{
                TileType::Empty=> {return false;},
                TileType::Numbered(_)=> {return true;}
            }
        }
        false
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.col >= 0
            && location.row >= 0
            && location.col < GRID_SIZE as i32
            && location.row < GRID_SIZE as i32
    }
}

impl<T: PartialEq + Eq> Index<&GridLocation> for Board<T> {
    type Output = Option<Arc::<RwLock::<T>>>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.grid[index.row as usize][index.col as usize]
    }
}

impl<T: PartialEq + Eq> IndexMut<&GridLocation> for Board<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.grid[index.row as usize][index.col as usize]
    }
}