use std::{
    collections::HashSet,
    ops::{Index,IndexMut},
};

use bevy::{prelude::*, utils::HashMap};

use crate::prelude::{TileType, BasicDirection};

pub const GRID_SIZE: u32 = 4;

#[derive(Resource)]
pub struct Grid {
    pub tile_entities: [[TileType; GRID_SIZE as usize]; GRID_SIZE as usize]
}

#[derive(Resource)]
pub struct ConnectedComponents {
    pub components: Vec<HashSet<GridLocation>>,
}

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug, Deref, DerefMut)]
pub struct GridLocation(pub IVec2);

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid>();
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            tile_entities: [[TileType::Empty; GRID_SIZE as usize]; GRID_SIZE as usize],
        }
    }
}

impl Grid {
    pub fn get_all_direct_neighbors(&self, location: &GridLocation) -> HashMap<BasicDirection, TileType>{
        let mut valid_neighbors:HashMap<BasicDirection, TileType>=HashMap::new();
        for dir in BasicDirection::get_directions_as_vec(){
            if let Some(neighbor_location) = self.neighbor_location(&location, &dir){
                valid_neighbors.insert(dir,self[location]);
            }
        }
        valid_neighbors
    }

    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> Option<GridLocation>{
        let optional_neighbor_location=match dir{
            BasicDirection::Up=>GridLocation::new(origin.x, origin.y+1),
            BasicDirection::Right=>GridLocation::new(origin.x+1, origin.y),
            BasicDirection::Down=>GridLocation::new(origin.x, origin.y-1),
            BasicDirection::Left=>GridLocation::new(origin.x-1, origin.y)
        };
        if self.occupied(&optional_neighbor_location){
            Some(optional_neighbor_location)
        }else{
            None
        }
    }

    pub fn occupied(&self, location: &GridLocation) -> bool {
        match self[location]{
            TileType::Empty=>false,
            TileType::Numbered(_)=>Grid::valid_index(location)
        }
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
    }
}

impl Index<&GridLocation> for Grid {
    type Output = TileType;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.tile_entities[index.x as usize][index.y as usize]
    }
}

impl IndexMut<&GridLocation> for Grid {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.tile_entities[index.x as usize][index.y as usize]
    }
}

impl GridLocation {
    pub fn new(x: i32, y: i32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Grid::valid_index(&location) {
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