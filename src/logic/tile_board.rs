use crate::{prelude::*, output::error_handler};

impl Board<Tile> {
    /// assumes one is empty
    pub fn switch_tiles_by_location(&mut self, first: &GridLocation, second: &GridLocation)
    -> Result<(), error_handler::TileMoveError>
    {
        if let None = self[first]{
            return Err(error_handler::TileMoveError::NoTileInCell(first.clone()));
        }
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
            let neighbor_location = self.neighbor_location(origin, &dir);
            if Board::<Tile>::valid_index(&neighbor_location) && !self.occupied(&neighbor_location)?{
                return Ok(Some(neighbor_location));
            }
        }
        Ok(None)
    }

    pub fn get_all_direct_neighbor_locations(&self, origin: &GridLocation) 
        -> HashMap<BasicDirection, GridLocation>
    {
        let mut valid_neighbors:HashMap<BasicDirection, GridLocation>=HashMap::new();
        for dir in BasicDirection::get_directions_as_vec(){
            if let Ok(Some(neighbor_location)) = self.occupied_neighbor_location(origin, &dir){
                valid_neighbors.insert(dir,neighbor_location);
            }
        }
        valid_neighbors
    }

    pub fn occupied_neighbor_location(
            &self, 
            origin: &GridLocation, 
            dir: &BasicDirection
        ) -> Result<Option<GridLocation>, error_handler::TileMoveError>
        {
        let neighbor_location = self.neighbor_location(origin, dir);
        if self.occupied(&neighbor_location)?{
            Ok(Some(neighbor_location))
        }else{
            Ok(None)
        }
    }

    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> GridLocation{
        match dir{
            BasicDirection::Up=>GridLocation::new(origin.row-1, origin.col),
            BasicDirection::Right=>GridLocation::new(origin.row, origin.col+1),
            BasicDirection::Down=>GridLocation::new(origin.row+1, origin.col),
            BasicDirection::Left=>GridLocation::new(origin.row, origin.col-1)
        }
    }

    pub fn occupied(&self, location: &GridLocation) -> Result<bool, error_handler::TileMoveError> {
        if let None = self[location]{
            return Err(error_handler::TileMoveError::NoTileInCell(location.clone()))
        }
        if Board::<Tile>::valid_index(location){
            match self[location].unwrap().tile_type{
                TileType::Empty=> {return Ok(false);},
                TileType::Numbered(_)=> {return Ok(true);}
            }
        }
        Ok(false)
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.col >= 0
            && location.row >= 0
            && location.col < GRID_SIZE as i32
            && location.row < GRID_SIZE as i32
    }
}