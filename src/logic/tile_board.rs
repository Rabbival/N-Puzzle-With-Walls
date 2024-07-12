use enum_iterator::all;
use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct TileBoard {
    /// even if the location is empty, TileBoard's location should have an empty tile (and NOT a None)
    pub grid: Grid<Tile>,
    pub empty_tile_locations: Vec<GridLocation>,
    ///appear as frozen to player
    pub ignore_player_input: bool,
}

//constructors
impl TileBoard {
    pub fn from_grid_and_empty_loc(
        grid: &Grid<Tile>,
        empty_tile_locations: &Vec<GridLocation>,
    ) -> Self {
        Self {
            grid: grid.clone(),
            empty_tile_locations: empty_tile_locations.clone(),
            ignore_player_input: true,
        }
    }

    pub fn from_grid(grid: &Grid<Tile>) -> Self{
        let mut newborn_self = Self {
            grid: grid.clone(),
            empty_tile_locations: vec![],
            ignore_player_input: true,
        };
        newborn_self.determine_empty_tile_locations_from_given_grid(grid);
        newborn_self
    }

    /// declares empty tiles' locations as last non-walls from the end
    pub fn try_from_solved_grid(
        grid: &Grid<Tile>,
        empty_tiles_count: u8,
    ) -> Result<Self, BoardGenerationError> {
        let mut newborn_self = Self {
            grid: grid.clone(),
            empty_tile_locations: vec![],
            ignore_player_input: true,
        };
        newborn_self.empty_locations_to_solved_default(empty_tiles_count)?;
        Ok(newborn_self)
    }

    /// looks for the empty tiles in the grid
    pub fn new(grid_side_length: u8) -> Self {
        Self {
            grid: Grid::new(grid_side_length),
            empty_tile_locations: vec![],
            ignore_player_input: true,
        }
    }
}

//creation helpers
impl TileBoard{
    pub fn determine_empty_tile_locations_from_given_grid(&mut self, grid: &Grid<Tile>){
        for (tile_location, new_empty_tile) in grid.iter(){
            if new_empty_tile.tile_type == TileType::Empty{
                let new_tile_index = new_empty_tile.index;
                let new_tile_index = self.empty_tile_locations.partition_point(
                    |empty_tile_location| {
                        if let Ok(Some(existing_tile)) = grid.get(empty_tile_location){
                            existing_tile.index < new_tile_index
                        }else{
                            false
                        }
                    }
                );
                self.empty_tile_locations.insert(new_tile_index, tile_location);
            }
        }
    }

    /// inserts empties without indexing them in the available (meaning not wall) locations from the end
    pub fn empty_locations_to_solved_default(
        &mut self,
        empty_tiles_count: u8,
    ) -> Result<(), BoardGenerationError> {
        self.empty_tile_locations = self.available_locations_from_the_end(empty_tiles_count)?;
        Ok(())
    }
    
    fn available_locations_from_the_end(
        &self,
        empty_tiles_count: u8,
    ) -> Result<Vec<GridLocation>, BoardGenerationError> {
        let mut empty_tile_locations = vec![];
        let mut reversed_iter = self.iter_filtered().rev();
        for _empty_tile in 0..empty_tiles_count {
            let next_from_last_available = reversed_iter.next();
            match next_from_last_available {
                Some((tile_location, _tile)) => empty_tile_locations.push(tile_location),
                None => return Err(BoardGenerationError::NotEnoughAvailableSpots),
            };
        }
        // we want them to appear in the same order they're indexed
        empty_tile_locations.reverse();
        Ok(empty_tile_locations)
    }
    
    pub fn spawn_walls_in_locations(&mut self, locations: &Vec<GridLocation>) 
        -> Result<(), GridError>
    {
        for location in locations {
            self.set(location, Tile::new(TileType::Wall))?;
        }
        Ok(())
    }

    pub fn spawn_empty_tiles(
        &mut self,
        applied_props: &BoardProperties,
        grid_side_length_u32: &u32,
    ) -> Result<(), GridError>
    {
        let mut empty_tile_counter = applied_props.empty_count;
        'outer_for: for i in (0..*grid_side_length_u32).rev() {
            for j in (0..*grid_side_length_u32).rev() {
                let location = GridLocation::new(i as i32, j as i32);
                if self.try_get_tiletype_in_location(&location)?.is_none() {
                    self.set(&location, Tile::new(TileType::Empty))?;
                    empty_tile_counter -= 1;
                    if empty_tile_counter == 0 {
                        break 'outer_for;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn spawn_numbered_uninitialized_tiles(
        &mut self,
        grid_side_length_u32: &u32,
    ) -> Result<(), GridError>
    {
        for i in 0..*grid_side_length_u32 {
            for j in 0..*grid_side_length_u32 {
                let location = GridLocation::new(i as i32, j as i32);
                if self.try_get_tiletype_in_location(&location)?.is_none() {
                    self.set(&location, Tile::new(TileType::Numbered))?
                }
            }
        }
        Ok(())
    }
}

impl TileBoard {
    pub fn index_all_tile_types(&mut self) {
        for tile_type in all::<TileType>() {
            self.index_tile_of_type(tile_type);
        }
    }

    /// provides indexes to a type of tile
    pub fn index_tile_of_type(&mut self, tile_type_to_index: TileType) {
        let only_that_type_iter = self
            .grid
            .iter_mut()
            .filter(|(_, tile_reference)| tile_reference.tile_type == tile_type_to_index)
            .map(|(_, optional_tile)| optional_tile);

        for (fixed_index, tile_of_type_to_index) in (0_u32..).zip(only_that_type_iter) {
            tile_of_type_to_index.index = fixed_index as usize;
        }
    }

    /// assumes at least one is empty, updates the empty_tile_locations index
    /// if both are empty, does nothing
    pub fn swap_tiles_by_location(
        &mut self,
        first: &GridLocation,
        second: &GridLocation,
    ) -> Result<(), TileMoveError> {
        let empty_tile_index;
        if let TileType::Empty = self.tiletype_in_location(first)? {
            if let TileType::Empty = self.tiletype_in_location(second)? {
                return Err(TileMoveError::TriedToSwitchEmptyWithEmpty);
            } else {
                empty_tile_index = self.get(first).unwrap().unwrap().index;
                self.empty_tile_locations[empty_tile_index] = *second;
            }
        } else {
            empty_tile_index = self.get(second).unwrap().unwrap().index;
            self.empty_tile_locations[empty_tile_index] = *first;
        }

        Ok(self.grid.swap_by_location(first, second)?)
    }

    pub fn tiletype_in_location(&self, location: &GridLocation)
    -> Result<TileType, TileBoardError>
    {
        match self.get(location)?{
            Some(tile_ref) => Ok(tile_ref.tile_type),
            None => Err(TileBoardError::NoTileInCell(*location))
        }
    }

    fn try_get_tiletype_in_location(&self, location: &GridLocation)
    -> Result<Option<TileType>, GridError>
    {
        match self.get(location)?{
            Some(tile_ref) => Ok(Some(tile_ref.tile_type)),
            None => Ok(None)
        }
    }
    
    pub fn try_get_all_empty_tiles(&self) -> Result<Vec<&Tile>, TileBoardError>{
        let mut empty_tiles_vec = vec!();
        for empty_tile_index in 0..self.empty_tile_locations.len(){
            empty_tiles_vec.push(self.try_get_empty_tile(empty_tile_index)?)
        }
        Ok(empty_tiles_vec)
    }

    /// if it gets an index out of empties bounds, sets the index to the last cell's
    pub fn try_get_empty_tile(&self, empty_tile_index: usize)
    -> Result<&Tile, TileBoardError>
    {
        let empty_tile_location = self.get_empty_tile_location(empty_tile_index);
        let cell_content = self.grid.get(empty_tile_location);
        match cell_content?{
            Some(tile) => Ok(tile),
            None => Err(TileBoardError::NoTileInCell(*empty_tile_location))
        }
    }

    /// if it gets an index out of empties bounds, sets the index to the last cell's
    pub fn get_empty_tile_location(&self, mut empty_tile_index: usize) -> &GridLocation {
        let empty_locations_count = self.empty_tile_locations.len();
        if empty_tile_index >= empty_locations_count {
            empty_tile_index = empty_locations_count - 1;
        }
        self.empty_tile_locations.get(empty_tile_index).unwrap()
    }
    
    pub fn get_direct_neighbors_of_empty(&self, empty_tile_index: usize) -> HashMap<BasicDirection, GridLocation> {
        self.get_neighbor_locations_of_type(
            self.get_empty_tile_location(empty_tile_index),
            TileType::Numbered
        )
    }

    pub fn get_empty_neighbors(&self, origin: &GridLocation) -> FoundEmptyNeighbors {
        let empty_neighbors =
            self.get_neighbor_locations_of_type(origin, TileType::Empty);
        let empty_neighbors_as_tiles: HashMap<BasicDirection, Tile> = 
            empty_neighbors.iter().map(|(direction, location)|{
                (*direction, *self.get(location).unwrap().unwrap())
            }).collect();
        FoundEmptyNeighbors::from_empty_neighbors_map(empty_neighbors_as_tiles)
    }

    pub fn get_neighbor_locations_of_type(
        &self,
        origin: &GridLocation,
        type_to_return: TileType
    ) -> HashMap<BasicDirection, GridLocation> 
    {
        let mut direct_neighbor_locations = self.grid.get_all_initialized_neighbor_locations(origin);
        for (dir, loc) in self.grid.get_all_initialized_neighbor_locations(origin) {
            if let Some(value_in_cell) = self.get(&loc).unwrap() {
                if value_in_cell.tile_type != type_to_return {
                    direct_neighbor_locations.remove(&dir);
                }
            }
        }
        direct_neighbor_locations
    }
}

// iterators
impl TileBoard {
    pub fn iter_filtered(&self) -> impl DoubleEndedIterator<Item = (GridLocation, &Tile)> + '_ {
        self.grid
            .iter()
            .filter(|(_, tile_reference)| tile_reference.tile_type != TileType::Wall)
    }
    
    pub fn walls_iter(&self) -> impl DoubleEndedIterator<Item = (GridLocation, &Tile)> + '_ {
        self.grid
            .iter()
            .filter(|(_, tile_reference)| tile_reference.tile_type == TileType::Wall)
    }
}

//from grid functions
impl TileBoard {
    pub fn get_side_length(&self) -> u8 {
        self.grid.get_side_length()
    }

    pub fn get(&self, location: &GridLocation) 
    -> Result<Option<&Tile>, GridError>
    {
        self.grid.get(location)
    }

    pub fn get_mut(&mut self, location: &GridLocation) 
    -> Result<Option<&mut Tile>, GridError>
    {
        self.grid.get_mut(location)
    }

    /// returns whether insertion was successful
    pub fn set(&mut self, location: &GridLocation, content: Tile) -> Result<(), GridError> {
        self.grid.set(location, content)
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, content: Tile)
    -> Result<Option<Tile>,  GridError>
    {
        self.grid.set_and_get_former(location, content)
    }

    pub fn is_tile_empty(&self, location: &GridLocation) 
    -> Result<bool, TileBoardError>
    {
        let tile_ref = self.none_check_get(location)?;
        match tile_ref.tile_type {
            TileType::Empty => {
                Ok(true)
            }
            TileType::Numbered | TileType::Wall => {
                Ok(false)
            }
        }
    }

    fn none_check_get(&self, location: &GridLocation) 
    -> Result<&Tile, TileBoardError>
    {
        match self.get(location)? {
            None => Err(TileBoardError::NoTileInCell(*location)),
            Some(tile_ref) => Ok(tile_ref),
        }
    }
}

impl Default for TileBoard {
    fn default() -> Self {
        Self::new(BoardSize::default().to_grid_side_length())
    }
}