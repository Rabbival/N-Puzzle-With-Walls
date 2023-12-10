use crate::{prelude::*, output::error_handler};

use std::ops::{Index,IndexMut};

#[derive(Component, Clone, Debug, Default)]
pub struct TileBoard {
    /// even if the location is empty, TileBoard's location should have an empty tile (and NOT a None)
    pub grid: InteriorMutGrid<Tile>,
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
}

//constructors
impl TileBoard{
    pub fn from_grid(grid: &InteriorMutGrid<Tile>)-> Self{
        Self { grid: grid.clone(), ..Default::default() }
    }
}

impl TileBoard {
    /// assumes one is empty
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation)
    -> Result<(), error_handler::TileMoveError>
    {
        self.none_check(first)?;
        self.none_check(second)?;
        if self[first].unwrap().tile_type==TileType::Empty{
            self.empty_tile_location=second.clone();
        }else{
            self.empty_tile_location=first.clone();
        }

        let temp_tile=self[first];
        self[first]=self[second];
        self[second]=temp_tile;
        Ok(())
    }

    pub fn get_direct_neighbors_of_empty(&self) -> HashMap<BasicDirection, GridLocation>{
        self.get_all_direct_neighbor_locations(&self.empty_tile_location) 
    }

    pub fn get_empty_neighbor(&self, origin: &GridLocation) 
    -> Result<Option<GridLocation>, error_handler::TileMoveError>
    {
        for dir in BasicDirection::get_directions_as_vec(){
            let neighbor_location = self.grid.neighbor_location(origin, &dir);
            if self.valid_index(&neighbor_location) && !self.occupied(&neighbor_location)?{
                return Ok(Some(neighbor_location));
            }
        }
        Ok(None)
    }

    pub fn get_all_direct_neighbor_locations(&self, origin: &GridLocation) 
        -> HashMap<BasicDirection, GridLocation>
    {
        self.grid.get_all_direct_neighbor_locations(origin)
    }

    pub fn occupied(&self, location: &GridLocation) -> Result<bool, error_handler::TileMoveError> {
        self.none_check(location)?;
        if self.valid_index(location){
            match self[location].unwrap().tile_type{
                TileType::Empty=> {return Ok(false);},
                TileType::Numbered(_)=> {return Ok(true);}
            }
        }
        Ok(false)
    }

    pub fn valid_index(&self, location: &GridLocation) -> bool {
        self.grid.valid_index(location)
    }

    fn none_check(&self, location: &GridLocation)-> Result<(), error_handler::TileMoveError>{
        match self[location] {
            None => Err(error_handler::TileMoveError::NoTileInCell(location.clone())),
            Some(_) => Ok(())
        }
    }
}

impl Index<&GridLocation> for TileBoard {
    type Output = Option<Tile>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<&GridLocation> for TileBoard {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.grid[index]
    }
}