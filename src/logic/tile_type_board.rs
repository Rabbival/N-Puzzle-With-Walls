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
    pub fn from_grid_and_empty_loc(
        grid: &Grid<TileType>,
        empty_tile_location: &GridLocation
    ) -> Self
    {
        Self { 
            grid: grid.clone(), 
            empty_tile_location: *empty_tile_location, 
            ignore_player_input: true
        }
    }

    ///puts empty tile at the last tile of the grid
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
    /// provides indexes to walls 
    pub fn index_walls(&mut self){
        let only_walls_iter = self.grid.iter_mut().filter(|(_, optional_tile)|{
            if let Some(tile) = optional_tile{
                if let TileType::Wall(_) = tile {
                    return true;
                }else{
                    return false;
                }
            }
            // shouldn't be, but in case it's a None
            false
        }).map(|(_, wall)|{
                wall
            }
        );
        let mut fixed_wall_index: u32 = 0;
        for wall in only_walls_iter{
            if let TileType::Wall(current_index)= wall.unwrap(){
                *current_index = fixed_wall_index;
            }
            fixed_wall_index += 1;
        }
    }

    /// assumes one is empty
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation)
    -> Result<(), error_handler::TileMoveError>
    {
        self.none_check(first)?;
        self.none_check(second)?;
        if let TileType::Empty(_) = self.get(first).unwrap() {
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
        self.grid.get_all_direct_neighbor_locations(&self.empty_tile_location) 
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

    /// only returns occupied ones that aren't walls
    pub fn get_direct_neighbor_locations_walls_excluded(&self, origin: &GridLocation) 
        -> HashMap<BasicDirection, GridLocation>
    {
        let mut direct_neighbor_locations 
            = self.grid.get_all_direct_neighbor_locations(origin);
        for (dir, loc) in self.grid.get_all_direct_neighbor_locations(origin){
            if let Some(value_in_cell) = self.grid.get(&loc){
                if let TileType::Wall(_) = value_in_cell{
                    direct_neighbor_locations.remove(&dir);
                }
            }
        }
        direct_neighbor_locations
    }

    fn none_check(&self, location: &GridLocation)-> Result<(), error_handler::TileMoveError>{
        match self.get(location) {
            None => Err(error_handler::TileMoveError::NoTileInCell(*location)),
            Some(_) => Ok(())
        }
    }
}

//manipulation (or short access) to the grid's functions
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

    /// removes and returns former, or None if there was none
    pub fn remove(&mut self, location: &GridLocation)-> Option<TileType> {
        if self.valid_index(location){
            self.grid.remove(location)
        }else{
            None
        }
    }

    // returns whether it's not empty
    pub fn occupied(&self, location: &GridLocation) 
        -> Result<bool, error_handler::TileMoveError> 
    {
        self.none_check(location)?;
        if self.valid_index(location){
            match self.get(location).unwrap(){
                TileType::Empty(_)=> {return Ok(false);},
                TileType::Numbered(_) | TileType::Wall(_) => {return Ok(true);}
            }
        }
        Ok(false)
    }

    pub fn valid_index(&self, location: &GridLocation) -> bool {
        self.grid.valid_index(location)
    }
}

// iterators
impl TileTypeBoard{
    pub fn iter_filtered(&self) -> impl Iterator<Item = (&GridLocation, Option<&TileType>)> + '_ {
        self.grid.iter().filter(|(_, optional_tile)|{
            if let Some(tile) = *optional_tile{
                if let TileType::Wall(_) = tile {
                    return false;
                }else{
                    return true;
                }
            }
            // shouldn't be, but in case it's a None
            false
        })
    }
}

impl Default for TileTypeBoard{
    fn default() -> Self {
        Self::new(DEFAULT_BOARD_SIDE_LENGTH)
    }
}