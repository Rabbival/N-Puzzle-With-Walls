use crate::{prelude::*, output::error_handler};

#[derive(Component, Clone, Debug)]
pub struct TileTypeBoard {
    /// even if the location is empty, TileTypeBoard's location should have an empty tile (and NOT a None)
    pub grid: Grid<Tile>,
    pub empty_tile_location: GridLocation,
    ///appear as frozen to player
    pub ignore_player_input: bool
}

//constructors
impl TileTypeBoard{
    pub fn from_grid_and_empty_loc(
        grid: &Grid<Tile>,
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
    pub fn from_grid(grid: &Grid<Tile>) -> Self{
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
    fn index_tile_of_type(&mut self, tile_type_to_index: TileType){
        let only_that_type_iter 
            = self.grid.iter_mut()
                .filter(|(_, tile_reference)|{
                    tile_reference.tile_type == tile_type_to_index
                })
                .map(|(_, optional_tile)|{ optional_tile });

        for (fixed_index, tile_of_type_to_index) 
            in (0_u32..).zip(only_that_type_iter)
        {
            tile_of_type_to_index.index = fixed_index as usize;
        }
    }

    /// assumes one is empty
    pub fn swap_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation)
    -> Result<(), error_handler::TileMoveError>
    {
        self.none_check(first)?;
        self.none_check(second)?;
        if self.get(first).unwrap().tile_type == TileType::Empty {
            self.empty_tile_location= *second;
        }else{
            self.empty_tile_location= *first;
        }

        if self.grid.swap_by_location(first, second){
            Ok(())   
        }else{
            Err(error_handler::TileMoveError::IndexOutOfGridBounds
                    (String::from("index out of grid bounds when tried to swap")))
        }
    }

    pub fn get_direct_neighbors_of_empty(&self) -> HashMap<BasicDirection, GridLocation>{
        self.grid.get_all_direct_neighbor_locations(&self.empty_tile_location) 
    }

    pub fn get_empty_neighbor(&self, origin: &GridLocation) 
    -> Result<Option<GridLocation>, error_handler::TileMoveError>
    {
        for dir in BasicDirection::get_directions_as_vec(){
            let neighbor_location = self.grid.neighbor_location(origin, &dir);
            self.none_check(&neighbor_location)?;
            if self.get(&neighbor_location).unwrap().tile_type == TileType::Empty{
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
                if TileType::Wall == value_in_cell.tile_type{
                    direct_neighbor_locations.remove(&dir);
                }
            }
        }
        direct_neighbor_locations
    }

    /// returns true if it was None and the value was inserted
    pub fn set_if_none(&mut self, location: &GridLocation, content: Tile) -> bool{
        if self.grid.get(location).is_none() {
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

    pub fn get(&self, location: &GridLocation) -> Option<&Tile> {
        self.grid.get(location)
    }

    pub fn get_mut(&mut self, location: &GridLocation) -> Option<&mut Tile> {
        self.grid.get_mut(location)
    }

    /// returns whether insertion was successful
    pub fn set(&mut self, location: &GridLocation, content: Tile) -> bool {
        self.grid.set(location, content)
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, content: Tile) -> Option<Tile>
    {
        self.grid.set_and_get_former(location, content)
    }

    /// returns an option with the previous value
    pub fn none(&mut self, location: &GridLocation) -> Option<Tile>
    {
        self.grid.set_none_get_former(location)
    }

    // returns whether it's not empty
    pub fn occupied(&self, location: &GridLocation) 
        -> Result<bool, error_handler::TileMoveError> 
    {
        self.none_check(location)?;
        if self.valid_index(location){
            match self.get(location).unwrap().tile_type{
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
    pub fn iter_filtered(&self) -> impl Iterator<Item = (GridLocation, &Tile)> + '_ {
        self.grid.iter().filter(|(_, tile_reference)|{
            tile_reference.tile_type != TileType::Wall
        })
    }
}

impl Default for TileTypeBoard{
    fn default() -> Self {
        Self::new(BoardSize::default().to_grid_side_length())
    }
}