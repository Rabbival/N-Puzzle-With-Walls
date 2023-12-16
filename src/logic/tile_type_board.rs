use std::ops::{Index,IndexMut};

use crate::{prelude::*, output::error_handler};

#[derive(Component, Clone, Debug)]
pub struct TileTypeBoard {
    /// even if the location is empty, TileTypeBoard's location should have an empty tile (and NOT a None)
    pub grid: Grid<TileType>,
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
}

//constructors
impl TileTypeBoard{
    pub fn new(grid_side_length: u8) -> Self {
        Self { 
            grid: Grid::new(grid_side_length), 
            empty_tile_location: GridLocation { 
                row: (grid_side_length-1) as i32, 
                col: (grid_side_length-1) as i32
            }, 
            ignore_player_input: true
        }
    }

    pub fn from_grid(grid: &Grid<TileType>)-> Self{
        let grid_side_length=grid.get_side_length();
        Self { 
            grid: grid.clone(), 
            empty_tile_location: GridLocation { row: (grid_side_length-1) as i32, col: (grid_side_length-1) as i32}, 
            ignore_player_input: true
        }
    }
}

impl TileTypeBoard {
    /// assumes one is empty
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation)
    -> Result<(), error_handler::TileMoveError>
    {
        self.none_check(first)?;
        self.none_check(second)?;
        if self[first].unwrap()==TileType::Empty{
            self.empty_tile_location=second.clone();
        }else{
            self.empty_tile_location=first.clone();
        }

        //can't std::swap because that would require two coexisting &mut self
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
            match self[location].unwrap(){
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

impl Index<&GridLocation> for TileTypeBoard {
    type Output = TileType;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<&GridLocation> for TileTypeBoard {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.grid[index]
    }
}