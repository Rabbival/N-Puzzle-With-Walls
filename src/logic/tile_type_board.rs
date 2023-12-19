use crate::{prelude::*, output::error_handler};

pub const DEFAULT_BOARD_SIDE_LENGTH: u8 = 4;

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
    pub fn from_grid(grid: &Grid<TileType>) -> Self{
        let grid_side_length = grid.get_side_length();
        Self { 
            grid: grid.clone(), 
            empty_tile_location: GridLocation { 
                row: (grid_side_length-1) as i32, 
                col: (grid_side_length-1) as i32
            }, 
            ignore_player_input: true
        }
    }

    pub fn new(grid_side_length: u8) -> Self{
        Self { 
            grid: Grid::new(grid_side_length), 
            empty_tile_location: GridLocation { 
                row: (grid_side_length-1) as i32, 
                col: (grid_side_length-1) as i32
            }, 
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
        if self.get(first).unwrap()==TileType::Empty{
            self.empty_tile_location= *second;
        }else{
            self.empty_tile_location= *first;
        }

        //can't std::swap because that would require two coexisting &mut self
        let originally_in_first=
            self.set_and_get_former(first, *self.get(second).unwrap());
        self.set(second, originally_in_first.unwrap());
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
            if neighbor_location == self.empty_tile_location{
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
            match self.get(location).unwrap(){
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
        match self.get(location) {
            None => Err(error_handler::TileMoveError::NoTileInCell(*location)),
            Some(_) => Ok(())
        }
    }
}

//shorter access to grid's basic functions
impl TileTypeBoard{
    pub fn get_side_length(&self)-> &u8 {
        self.grid.get_side_length()
    }

    pub fn get(&self, location: &GridLocation) -> Option<&TileType> {
        self.grid.get(location)
    }

    pub fn get_mut(&mut self, location: &GridLocation) -> Option<&mut TileType> {
        self.grid.get_mut(location)
    }

    /// returns whether insertion was successful
    pub fn set(&mut self, location: &GridLocation, value: TileType) -> bool {
        self.grid.set(location, value)
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, value: TileType)-> Option<TileType>{
        self.grid.set_and_get_former(location, value)
    }
}

impl Default for TileTypeBoard{
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_SIDE_LENGTH)
    }
}