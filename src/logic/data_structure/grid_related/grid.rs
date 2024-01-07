use crate::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Grid<T: Clone> {
    grid_side_length: u8,
    grid: Vec<Option<T>>
}

//graph travelling functions
impl<T: Clone> Grid<T>{
    // pub fn get_spanning_tree(&self) -> bool {
        
    // }

    pub fn is_connected_graph(&self) -> bool {
        let mut cells_locations_with_added_mark: HashMap<GridLocation, bool>=
            self
            .iter()
            .map(|(location, _)| {
                (location, false)
            })
            .collect();
        let first_location 
            = cells_locations_with_added_mark.get_key_value_mut(
                &self.iter().next().unwrap().0
            ).unwrap();
        *first_location.1 = true; //already added
        let mut locations_to_visit = vec![*first_location.0];
        let mut cells_visited_counter = 0;
        while ! locations_to_visit.is_empty(){
            self.depth_first_count(
                &mut locations_to_visit,
                &mut cells_locations_with_added_mark
            );
            cells_visited_counter += 1;
        }

        //check that we found everything that's defined (and not None)
        cells_visited_counter == self.iter().collect::<Vec<_>>().len() as u32
    }

    /// assumes vec not to be empty and hash-map to have all locations in vec
    fn depth_first_count(
        &self,
        locations_to_visit: &mut Vec<GridLocation>,
        cells_locations_with_added_mark: &mut HashMap<GridLocation, bool>
    ){
        let next_tile_to_check = locations_to_visit.pop().unwrap();
        let next_tile_neighbors 
            = self.get_all_direct_neighbor_locations(&next_tile_to_check);
        let mut new_locations_to_visit : Vec<GridLocation>
            = next_tile_neighbors
                .values()
                //only add the ones not yet visited
                .filter(|next_tile_neighbor_location|{
                    ! *cells_locations_with_added_mark.get(*next_tile_neighbor_location).unwrap()
                })
                .copied()
                .collect();
        for new_location in new_locations_to_visit.clone(){
            let addeded_mark_for_new_location
                = cells_locations_with_added_mark.get_mut(&new_location).unwrap();
            *addeded_mark_for_new_location = true;
        }
        locations_to_visit.append(&mut new_locations_to_visit);
    }
}

impl<T: Clone> Grid<T>{
    /// only returns occupied ones 
    pub fn get_all_direct_neighbor_locations(&self, origin: &GridLocation) 
    -> HashMap<BasicDirection, GridLocation>
    {
        let mut valid_neighbors:HashMap<BasicDirection, GridLocation>=HashMap::new();
        for dir in BasicDirection::get_directions_as_vec(){
            if let Some(neighbor_location) = self.occupied_neighbor_location(origin, &dir){
                valid_neighbors.insert(dir,neighbor_location);
            }
        }
        valid_neighbors
    }

    fn occupied_neighbor_location(
            &self, 
            origin: &GridLocation, 
            dir: &BasicDirection
        ) -> Option<GridLocation>
        {
        let neighbor_location = self.neighbor_location(origin, dir);
        if self.occupied(&neighbor_location){
            Some(neighbor_location)
        }else{
            None
        }
    }

    //returns a location without checking if it's valid
    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> GridLocation{
        match dir{
            BasicDirection::Up=>GridLocation::new(origin.row-1, origin.col),
            BasicDirection::Right=>GridLocation::new(origin.row, origin.col+1),
            BasicDirection::Down=>GridLocation::new(origin.row+1, origin.col),
            BasicDirection::Left=>GridLocation::new(origin.row, origin.col-1)
        }
    }
}

// get a group of tiles as vector
impl<T: Clone> Grid<T>{
    pub fn corner_locations(&self) -> Vec<GridLocation>{
        let end_of_line = (self.grid_side_length-1) as i32;
        vec![
            GridLocation::new(0, 0),
            GridLocation::new(0, end_of_line),
            GridLocation::new(end_of_line, 0),
            GridLocation::new(end_of_line, end_of_line),
        ]
    }

    pub fn edges_without_corners_locations(&self) -> Vec<GridLocation>{
        let end_of_line = (self.grid_side_length-1) as i32;
        let mut edge_vector = vec![];
        for i in 1..end_of_line{
            edge_vector.push(GridLocation { row: 0, col: i }); //upper line
            edge_vector.push(GridLocation { row: end_of_line, col: i }); //buttom line
            edge_vector.push(GridLocation { row: i, col: 0 }); //left line
            edge_vector.push(GridLocation { row: i, col: end_of_line }); //right line
        }
        edge_vector
    }

    pub fn all_locations_no_edges(&self) -> Vec<GridLocation>{
        let end_of_line = (self.grid_side_length-1) as i32;
        let mut edge_vector = vec![];
        for col in 1..end_of_line{
            for row in 1..end_of_line{
                edge_vector.push(GridLocation { row, col }); 
            }
        }
        edge_vector
    }

    /// only returns ones that aren't None
    pub fn all_initialized_locations_as_vec(&self) -> Vec<GridLocation>{
        let mut initialized_locations_vector = vec![];
        for col in 0..(self.grid_side_length as i32){
            for row in 0..(self.grid_side_length as i32){
                let location = GridLocation { row, col };
                if self.get(&location).is_some(){
                    initialized_locations_vector.push(location);
                }
            }
        }
        initialized_locations_vector
    }
}

//basics
impl<T: Clone> Grid<T> {
    /// initializes to None
    pub fn new(grid_side_length: u8) -> Self {
        let mut grid : Vec::<Option<T>> = vec![];
        for _ in 0..(grid_side_length*grid_side_length){
            grid.push(None);
        }
        Self {
            grid_side_length,
            grid
        }
    }

    pub fn get_side_length(&self)-> &u8 {
        &self.grid_side_length
    }

    pub fn get(&self, location: &GridLocation) -> Option<&T> {
        if self.valid_index(location){
            self.grid.get(self.location_to_index(location))?.as_ref()
        }else{
            None
        }
    }

    pub fn get_mut(&mut self, location: &GridLocation) -> Option<&mut T> {
        if self.valid_index(location){
            let location_index = self.location_to_index(location);
            self.grid.get_mut(location_index)?.as_mut()
        }else{
            None
        }
    }

    /// returns whether insertion was successful
    pub fn set(&mut self, location: &GridLocation, value: T) -> bool {
        if self.valid_index(location){
            self.grid[location.to_index(self.grid_side_length)] = Some(value);
            return true;
        }
        false
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, value: T)-> Option<T>{
        if self.valid_index(location){
            let former = self.grid[location.to_index(self.grid_side_length)].clone();
            self.set(location, value);
            former
        }else{
            None
        }
    }

    /// returns an option with the previous value
    pub fn set_none_get_former(&mut self, location: &GridLocation)-> Option<T>{
        if self.valid_index(location){
            let former = self.grid[location.to_index(self.grid_side_length)].clone();
            self.grid[location.to_index(self.grid_side_length)] = None;
            former
        }else{
            None
        }
    }

    /// returns true if the operation was successful
    pub fn swap_by_location(&mut self, first: &GridLocation, second: &GridLocation) -> bool {
        if self.valid_index(first) && self.valid_index(second){
            let first_location_index = self.location_to_index(first);
            let second_location_index = self.location_to_index(second);
            self.grid.swap(first_location_index, second_location_index);
            return true;
        }
        false
    }

    /// also returns false if the location is invalid, so remember to check that if relevant
    pub fn occupied(&self, location: &GridLocation) -> bool {
        self.get(location).is_some()
    }

    /// object function in case we'd want to have grids of different sizes
    pub fn valid_index(&self, location: &GridLocation) -> bool {
        location.col >= 0
            && location.row >= 0
            && location.col < self.grid_side_length as i32
            && location.row < self.grid_side_length as i32
    }

    pub fn location_from_index(&self, index: usize)-> GridLocation {
        GridLocation::from_index(index as u8, self.grid_side_length)
    }

    pub fn location_to_index(&self, location: &GridLocation) -> usize{
        location.to_index(self.grid_side_length)
    }
}

//iterators and filter
impl<T: Clone> Grid<T> {
    /// returns occupied (initialized) cells' references only
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (GridLocation, &T)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, optional_value)|
                optional_value.is_some())
            .map(|(index, optional_value)|{
                (self.location_from_index(index), optional_value.as_ref().unwrap())
            })
    }

    /// returns occupied (initialized) cells' references only
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (GridLocation, &mut T)> + '_ {
        self.grid
            .iter_mut()
            .enumerate()
            .filter(|(_, optional_value)|
                optional_value.is_some())
            .map(|(index, optional_value)|{
                (GridLocation::from_index(index as u8, self.grid_side_length), optional_value.as_mut().unwrap())
            })
    }
}

impl<T: PartialEq + Eq + Clone> PartialEq for Grid<T>{
    fn eq(&self, other: &Self) -> bool {
        let mut all_cells_are_equal=true;
        let self_iter=self.iter();
        let other_iter=other.iter();
        for ((_ , self_value), ( _ , other_value)) in self_iter.zip(other_iter) {
            if self_value != other_value{
                all_cells_are_equal=false;
                break;
            }
        }
        all_cells_are_equal
    }
}
impl<T: PartialEq + Eq + Clone> Eq for Grid<T>{}