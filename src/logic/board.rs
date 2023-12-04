use std::ops::{Index,IndexMut};

use bevy::{prelude::*, utils::HashMap};

use crate::prelude::{Tile, TileType, BasicDirection};

pub const GRID_SIZE: u32 = 4;

#[derive(Resource, PartialEq, Eq, Clone)]
pub struct Board {
    pub grid: [[Tile; GRID_SIZE as usize]; GRID_SIZE as usize]
}

#[derive(Component, Default, Eq, PartialEq, Hash, Clone, Copy, Debug, Deref, DerefMut)]
pub struct GridLocation(pub IVec2);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>();
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            grid: [[Tile::default(); GRID_SIZE as usize]; GRID_SIZE as usize],
        }
    }
}

impl Board {
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation){
        let temp_tile=self[first];
        self[first]=self[second];
        self[second]=temp_tile;
    }

    pub fn get_all_direct_neighbor_locations(&self, location: &GridLocation) 
        -> HashMap<BasicDirection, GridLocation>
    {
        let mut valid_neighbors:HashMap<BasicDirection, GridLocation>=HashMap::new();
        for dir in BasicDirection::get_directions_as_vec(){
            if let Some(neighbor_location) = self.neighbor_location(&location, &dir){
                valid_neighbors.insert(dir,neighbor_location);
            }
        }
        valid_neighbors
    }

    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> Option<GridLocation>{
        let neighbor_location=match dir{
            BasicDirection::Up=>GridLocation::new(origin.x, origin.y+1),
            BasicDirection::Right=>GridLocation::new(origin.x+1, origin.y),
            BasicDirection::Down=>GridLocation::new(origin.x, origin.y-1),
            BasicDirection::Left=>GridLocation::new(origin.x-1, origin.y)
        };
        if self.occupied(&neighbor_location){
            Some(neighbor_location)
        }else{
            None
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
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
    }
}

impl Index<&GridLocation> for Board {
    type Output = Tile;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.grid[index.x as usize][index.y as usize]
    }
}

impl IndexMut<&GridLocation> for Board {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.grid[index.x as usize][index.y as usize]
    }
}


impl GridLocation {
    pub fn new(x: i32, y: i32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Board::valid_index(&location) {
            Some(location)
        } else {
            None
        }
    }
}

impl From<IVec2> for GridLocation {
    fn from(value: IVec2) -> Self {
        GridLocation(value)
    }
}