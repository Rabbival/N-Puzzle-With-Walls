use std::{
    collections::HashSet,
    ops::{Index,IndexMut}
};

use bevy::prelude::*;

pub const GRID_SIZE: usize = 4;

#[derive(Resource)]
pub struct Grid {
    pub tile_entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE]
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
            tile_entities: [[None; GRID_SIZE]; GRID_SIZE],
        }
    }
}

impl Grid {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
    }
}

//iterator implementation
impl Grid{
    pub fn iter(&self) -> impl Iterator<Item = (Entity, GridLocation)> + '_ {
        self.entities
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_i, entity)| entity.is_some())
            .map(|(i, entity)| {
                (
                    entity.unwrap(),
                    GridLocation::new(i as u32 / GRID_SIZE as u32, i as u32 % GRID_SIZE as u32),
                )
            })
    }
}

impl Index<&GridLocation> for Grid {
    type Output = Option<Entity>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

impl IndexMut<&GridLocation> for Grid {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.entities[index.x as usize][index.y as usize]
    }
}

impl GridLocation {
    pub fn new(x: u32, y: u32) -> Self {
        GridLocation(IVec2::new(x as i32, y as i32))
    }

    pub fn from_world(position: Vec2) -> Option<Self> {
        let position = position + Vec2::splat(0.5);
        let location = GridLocation(IVec2::new(position.x as i32, position.y as i32));
        if Grid::<()>::valid_index(&location) {
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