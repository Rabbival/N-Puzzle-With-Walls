use crate::{prelude::*, output::error_handler};

pub const DEFAULT_BOARD_SIDE_LENGTH: u8 = 4;

#[derive(Component, Clone, Debug)]
pub struct TileTypeBoard {
    /// even if the location is empty, TileTypeBoard's location should have an empty tile (and NOT a None)
    pub grid: Grid<IndexedValue<TileType>>,
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
}

//constructors
impl TileTypeBoard{
    pub fn from_grid_and_empty_loc(
        grid: &Grid<IndexedValue<TileType>>,
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
    pub fn from_grid(grid: &Grid<IndexedValue<TileType>>) -> Self{
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
    pub fn index_all_tile_types(&mut self){
        for tile_type in TileType::get_tile_types_as_vec(){
            self.index_tile_of_type(tile_type);
        }
    }

    /// provides indexes to a type of tile
    pub fn index_tile_of_type(&mut self, tile_type_to_index: TileType){
        let only_that_type_iter = self.grid.iter_mut().filter(|(_, optional_tile)|{
            if let Some(tile_type_in_tile) = *optional_tile{
                if tile_type_in_tile.value == &tile_type_to_index {
                    return true;
                }else{
                    return false;
                }
            }
            // shouldn't be, but in case it's a None
            false
        }).map(|(_, indexed_value)|{
                indexed_value
            }
        );
        let mut fixed_index: u32 = 0;
        for tile_of_type_to_index in only_that_type_iter{
            tile_of_type_to_index.unwrap().index = fixed_index as usize;
            fixed_index += 1;
        }
    }

    /// assumes one is empty
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation)
    -> Result<(), error_handler::TileMoveError>
    {
        self.none_check(first)?;
        self.none_check(second)?;
        if self.get(first).unwrap().value == TileType::Empty {
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
                if TileType::Wall == value_in_cell.value{
                    direct_neighbor_locations.remove(&dir);
                }
            }
        }
        direct_neighbor_locations
    }

    /// returns true if it was None and the value was inserted
    pub fn set_if_empty(&mut self, location: &GridLocation, content: IndexedValue<TileType>) -> bool{
        if let None = self.grid.get(location){
            self.grid.set(location, content);
            return true;
        }
        false
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

    pub fn get(&self, location: &GridLocation) -> Option<&IndexedValue<TileType>> {
        self.grid.get(location)
    }

    pub fn get_mut(&mut self, location: &GridLocation) -> Option<&mut IndexedValue<TileType>> {
        self.grid.get_mut(location)
    }

    /// returns whether insertion was successful
    pub fn set(&mut self, location: &GridLocation, content: IndexedValue<TileType>) -> bool {
        self.grid.set(location, content)
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, content: IndexedValue<TileType>)
    -> Option<IndexedValue<TileType>>
    {
        self.grid.set_and_get_former(location, content)
    }

    /// removes and returns former, or None if there was none
    pub fn remove(&mut self, location: &GridLocation)-> Option<IndexedValue<TileType>> {
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
            match self.get(location).unwrap().value{
                TileType::Empty => {return Ok(false);},
                TileType::Numbered | TileType::Wall => {return Ok(true);}
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
    pub fn iter_filtered(&self) -> impl Iterator<Item = (&GridLocation, Option<&IndexedValue<TileType>>)> + '_ {
        self.grid.iter().filter(|(_, optional_tile)|{
            if let Some(tile) = *optional_tile{
                if tile.value == TileType::Wall {
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