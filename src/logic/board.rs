use std::ops::{Index,IndexMut};

use bevy::{prelude::*, utils::HashMap};

use crate::prelude::*;

pub const GRID_SIZE: u32 = 4;

#[derive(Component, Clone, Debug)]
pub struct Board {
    pub grid: [[Tile; GRID_SIZE as usize]; GRID_SIZE as usize],
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        let mut all_cells_are_equal=true;
        for row_index in 0..GRID_SIZE{
            for col_index in 0..GRID_SIZE{
                let location=GridLocation::new(row_index as i32, col_index as i32);
                if self[&location].tile_type != other[&location].tile_type{
                    all_cells_are_equal=false;
                    break;
                }
            }
        }
        all_cells_are_equal
    }
}
impl Eq for Board{}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[Tile::default(); GRID_SIZE as usize]; GRID_SIZE as usize],
            empty_tile_location: GridLocation::default(),
            ignore_player_input: true
        }
    }
}

impl Board {
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation){
        let temp_tile=self[first];
        self[first]=self[second];
        self[second]=temp_tile;
    }

    pub fn get_empty_neighbor(&mut self, origin: &GridLocation) -> Option<GridLocation>{
        for dir in BasicDirection::get_directions_as_vec(){
            let neighbor_location = self.neighbor_location(origin, &dir);
            if Board::valid_index(&neighbor_location) && !self.occupied(&neighbor_location){
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
        if Board::valid_index(location){
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

impl Index<&GridLocation> for Board {
    type Output = Tile;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.grid[index.row as usize][index.col as usize]
    }
}

impl IndexMut<&GridLocation> for Board {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.grid[index.row as usize][index.col as usize]
    }
}