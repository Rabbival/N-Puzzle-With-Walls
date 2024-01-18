use crate::{input::move_request, output::error_handler, prelude::*};

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

    /// declares empty tiles' locations as last avaliable from the end
    pub fn from_grid(
        grid: &Grid<Tile>,
        empty_tiles_count: u8,
    ) -> Result<Self, error_handler::BoardGenerationError> {
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

    /// inserts empties without indexing them in the available (meaning not wall) locations from the end
    pub fn empty_locations_to_solved_default(
        &mut self,
        empty_tiles_count: u8,
    ) -> Result<(), error_handler::BoardGenerationError> {
        self.empty_tile_locations = self.available_locations_from_the_end(empty_tiles_count)?;
        Ok(())
    }

    /// returns a vector with available places from the end
    fn available_locations_from_the_end(
        &mut self,
        empty_tiles_count: u8,
    ) -> Result<Vec<GridLocation>, error_handler::BoardGenerationError> {
        let mut empty_tile_locations = vec![];
        let mut reversed_iter = self.iter_filtered().rev();
        for _empty_tile in 0..empty_tiles_count {
            let next_from_last_avaliable = reversed_iter.next();
            match next_from_last_avaliable {
                Some((tile_location, _tile)) => empty_tile_locations.push(tile_location),
                None => return Err(error_handler::BoardGenerationError::NotEnoughAvailableSpots),
            };
        }
        // we want them to appear in the same order they're indexed
        empty_tile_locations.reverse();
        Ok(empty_tile_locations)
    }
}

impl TileBoard {
    pub fn index_all_tile_types(&mut self) {
        for tile_type in TileType::get_tile_types_as_vec() {
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
    ) -> Result<(), error_handler::TileMoveError> {
        let first_tile_type = self.tiletype_in_location(first);
        let second_tile_type = self.tiletype_in_location(second);
        if first_tile_type.is_none() {
            return Err(error_handler::TileMoveError::NoTileInCell(*first));
        }
        if second_tile_type.is_none() {
            return Err(error_handler::TileMoveError::NoTileInCell(*second));
        }

        let empty_tile_index;
        if let TileType::Empty = first_tile_type.unwrap() {
            if let TileType::Empty = second_tile_type.unwrap() {
                return Err(error_handler::TileMoveError::TriedToSwitchEmptyWithEmpty);
            } else {
                empty_tile_index = self.get(first).unwrap().index;
                self.empty_tile_locations[empty_tile_index] = *second;
            }
        } else {
            empty_tile_index = self.get(second).unwrap().index;
            self.empty_tile_locations[empty_tile_index] = *first;
        }

        if self.grid.swap_by_location(first, second).is_ok() {
            Ok(())
        } else {
            Err(error_handler::TileMoveError::GridError(GridError::InvalidIndex(())))
        }
    }

    pub fn tiletype_in_location(&self, location: &GridLocation) 
    -> Result<Option<TileType>, error_handler::GridError> 
    {
        match self.get(location)?{
            Some(tile_ref) => Ok(Some(tile_ref.tile_type)),
            None => Ok(None)
        }
    }

    /// if it gets an index out of empties bounds, sets the index to the last cell's
    pub fn get_empty_tile(&self, empty_tile_index: usize) 
    -> Result<Option<&Tile>, error_handler::GridError> 
    {
        let empty_tile_location = self.get_empty_tile_location(empty_tile_index);
        self.grid.get(empty_tile_location)
    }

    /// if it gets an index out of empties bounds, sets the index to the last cell's
    pub fn get_empty_tile_location(&self, mut empty_tile_index: usize) -> &GridLocation {
        let empty_locations_count = self.empty_tile_locations.len();
        if empty_tile_index >= empty_locations_count {
            empty_tile_index = empty_locations_count - 1;
        }
        self.empty_tile_locations.get(empty_tile_index).unwrap()
    }

    /// if it gets an index out of empties bounds, sets the index to the last cell's
    pub fn get_direct_neighbors_of_empty(
        &self,
        mut empty_tile_index: usize,
    ) -> HashMap<BasicDirection, GridLocation> {
        let empty_locations_count = self.empty_tile_locations.len();
        if empty_tile_index >= empty_locations_count {
            empty_tile_index = empty_locations_count - 1;
        }
        self.grid.get_all_occupied_neighbor_locations(
            self.empty_tile_locations.get(empty_tile_index).unwrap(),
        )
    }

    pub fn move_request_from_clicked_tile(
        &self,
        origin: &GridLocation,
    ) -> Result<Option<move_request::MoveRequest>, error_handler::TileMoveError> {
        let direct_neighbor_locations_walls_excluded = 
            self.get_direct_neighbor_locations_walls_excluded(origin);
        for (neighbor_direction, neighbor_location)
            in direct_neighbor_locations_walls_excluded
        {
            if let Some(tile_in_cell) = self.get(&neighbor_location).unwrap() {
                if tile_in_cell.tile_type == TileType::Empty {
                    return Ok(Some(move_request::MoveRequest {
                        move_neighbor_from_direction: neighbor_direction.opposite_direction(),
                        empty_tile_index: Some(tile_in_cell.index),
                    }));
                }
            }
        }
        Ok(None)
    }

    pub fn get_direct_neighbor_locations_walls_excluded(
        &self,
        origin: &GridLocation,
    ) -> HashMap<BasicDirection, GridLocation> {
        let mut direct_neighbor_locations = self.grid.get_all_occupied_neighbor_locations(origin);
        for (dir, loc) in self.grid.get_all_occupied_neighbor_locations(origin) {
            if let Some(value_in_cell) = self.get(&loc).unwrap() {
                if TileType::Wall == value_in_cell.tile_type {
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
}

//from grid functions
impl TileBoard {
    pub fn get_side_length(&self) -> &u8 {
        self.grid.get_side_length()
    }

    pub fn get(&self, location: &GridLocation) 
    -> Result<Option<&Tile>, error_handler::GridError> 
    {
        self.grid.get(location)
    }

    pub fn get_mut(&mut self, location: &GridLocation) 
    -> Result<Option<&mut Tile>, error_handler::GridError> 
    {
        self.grid.get_mut(location)
    }

    /// returns whether insertion was successful
    pub fn set(&mut self, location: &GridLocation, content: Tile) -> Result<(), error_handler::GridError> {
        self.grid.set(location, content)
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, content: Tile)
    -> Result<Option<Tile>,  error_handler::GridError>
    {
        self.grid.set_and_get_former(location, content)
    }

    pub fn empty_tile(&self, location: &GridLocation) 
    -> Result<bool, error_handler::TileMoveError> 
    {
        let tile_ref = self.none_check_get(location)?;
        match tile_ref.tile_type {
            TileType::Empty => {
                return Ok(true);
            }
            TileType::Numbered | TileType::Wall => {
                return Ok(false);
            }
        }
    }

    fn none_check_get(&self, location: &GridLocation) 
    -> Result<&Tile, error_handler::TileMoveError> 
    {
        match wrap_if_error(self.get(location))? {
            None => Err(error_handler::TileMoveError::NoTileInCell(*location)),
            Some(tile_ref) => Ok(tile_ref),
        }
    }

    fn none_check_get_mut(&self, location: &GridLocation) 
    -> Result<&Tile, error_handler::TileMoveError> 
    {
        match wrap_if_error(self.get_mut(location))? {
            None => Err(error_handler::TileMoveError::NoTileInCell(*location)),
            Some(mut_tile_ref) => Ok(mut_tile_ref),
        }
    }
}

impl Default for TileBoard {
    fn default() -> Self {
        Self::new(BoardSize::default().to_grid_side_length())
    }
}

/// I don't use it automatically inside the get set etc functions
/// since it they might have nothing to do with moving tiles
fn wrap_if_error<T>(result: Result<T, error_handler::GridError>) 
-> Result<T, error_handler::TileBoardError>{
    match result {
        Err(grid_error) => {
            Err(error_handler::TileBoardError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}
