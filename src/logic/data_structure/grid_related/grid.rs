use crate::{prelude::*, output::error_handler};
use crate::logic::data_structure::util_functions;

#[derive(Component, Clone, Debug)]
pub struct Grid<T: Clone> {
    grid_side_length: u8,
    grid: Vec<Option<T>>,
}

//grid travelling functions
impl<T: Clone> Grid<T> {
    /// when we travel the graph and get back to a node we've already visited
    /// we'll mark it and all the nodes after it as a part of a cycle
    pub fn all_nodes_in_cycles(&self) -> Result<bool, error_handler::DataStructError<GridLocation>>{
        let grid_traveller = GridTraveller::from_grid(self, GridTravellerType::DFS);
        let mut grid_tree = GridTree::from_root(grid_traveller.locations_to_visit[0]);
        let mut locations_not_in_cycle = LinkedList::<GridLocation>::new();

        for location_and_unadded_neighbors in grid_traveller {
            self.progress_traveller_and_update_cycles(
                location_and_unadded_neighbors,
                &mut locations_not_in_cycle,
                &mut grid_tree
            )?;
        }
        Ok(locations_not_in_cycle.is_empty())
    }

    //NTS: I want to make sure it doesn't ask the parent
    // must take into consideration it can also be null in case we check the root
    fn progress_traveller_and_update_cycles(
        &self,
        location_and_unadded_neighbors: LocationAndUnaddedNeighbors,
        locations_not_in_cycle: &mut LinkedList<GridLocation>,
        grid_tree: &mut GridTree,
    ) -> Result<(), error_handler::DataStructError<GridLocation>>
    {
        let just_visited_location = location_and_unadded_neighbors.just_visited_location;
        let just_added_neighbors = location_and_unadded_neighbors.just_added_neighbors.clone();
        let last_visited_location_tree_node = grid_tree.get_grid_tree_node(&just_visited_location);
        let last_visited_location_parent = last_visited_location_tree_node.unwrap().parent_location;
        locations_not_in_cycle.push(just_visited_location);
        for neighbor in just_added_neighbors {
            if let Err(tree_error) = grid_tree.insert_leaf(neighbor, Some(just_visited_location)){
                return Err(DataStructError::GridTreeError(tree_error));
            }
        }


        info!("locations not in cycle: {:?}", locations_not_in_cycle);


        let already_visited_neighbors =
            self.find_already_visited_neighbors(&location_and_unadded_neighbors)?;

        // if we got to a place already visited, it closes a cycle
        for already_visited_neighbor in already_visited_neighbors{


            info!("ordered to remove: {:?}", already_visited_neighbor);


            locations_not_in_cycle.remove_by_value(&already_visited_neighbor);
        }
        Ok(())
    }

    fn find_already_visited_neighbors(
        &self,
        location_and_unadded_neighbors: &LocationAndUnaddedNeighbors
    ) -> Result<Vec<GridLocation>,error_handler::DataStructError<GridLocation>> {
        let occupied_neighbors_and_directions_of_last_visited_location =
            self.get_all_occupied_neighbor_locations(&location_and_unadded_neighbors.just_visited_location);
        let mut unvisited_neighbors_of_just_visited : Vec<GridLocation> =
            occupied_neighbors_and_directions_of_last_visited_location.values()
                .map(|neighbor_location| {
                    *neighbor_location
                }).collect();
        for just_added_neighbor in location_and_unadded_neighbors.just_added_neighbors.clone(){
            let optional_just_added_location = util_functions::remove_by_value(
                &just_added_neighbor,
                &mut unvisited_neighbors_of_just_visited
            );
            if optional_just_added_location.is_none(){
                return Err(error_handler::DataStructError::ItemNotFound(just_added_neighbor))
            }
        }
        Ok(unvisited_neighbors_of_just_visited)
    }

    pub fn get_spanning_tree(&self, traveller_type: GridTravellerType) -> Result<GridTree, GridTreeError> {
        let grid_traveller = GridTraveller::from_grid(self, traveller_type);
        let mut grid_tree = GridTree::from_root(grid_traveller.locations_to_visit[0]);
        for location_and_neighbors in grid_traveller {
            for neighbor in location_and_neighbors.just_added_neighbors {
                grid_tree.insert_leaf
                    (neighbor, Some(location_and_neighbors.just_visited_location))?;
            }
        }
        Ok(grid_tree)
    }

    pub fn is_connected_graph(&self) -> bool {
        let mut traveller = 
            GridTraveller::from_grid(self, GridTravellerType::default());
        let mut tile_counter = 0;
        while traveller.next().is_some() {
            tile_counter += 1;
        }
        //check that we found everything that's defined (and not None)
        tile_counter == self.iter().count() as u32
    }
}

impl<T: Clone> Grid<T> {
    pub fn get_all_occupied_neighbor_locations(
        &self,
        origin: &GridLocation,
    ) -> HashMap<BasicDirection, GridLocation> 
    {
        let mut valid_neighbors: HashMap<BasicDirection, GridLocation> = HashMap::new();
        for dir in BasicDirection::get_directions_as_vec() {
            if let Some(neighbor_location) = self.occupied_neighbor_location(origin, &dir) {
                valid_neighbors.insert(dir, neighbor_location);
            }
        }
        valid_neighbors
    }

    fn occupied_neighbor_location(
        &self,
        origin: &GridLocation,
        dir: &BasicDirection,
    ) -> Option<GridLocation> {
        let neighbor_location = self.neighbor_location(origin, dir);
        let get_result = self.get(&neighbor_location);
        if get_result.is_ok() && get_result.unwrap().is_some() {
            Some(neighbor_location)
        } else {
            None
        }
    }

    /// returns a location without checking if it's valid
    pub fn neighbor_location(&self, origin: &GridLocation, dir: &BasicDirection) -> GridLocation {
        match dir {
            BasicDirection::Up => GridLocation::new(origin.row - 1, origin.col),
            BasicDirection::Right => GridLocation::new(origin.row, origin.col + 1),
            BasicDirection::Down => GridLocation::new(origin.row + 1, origin.col),
            BasicDirection::Left => GridLocation::new(origin.row, origin.col - 1),
        }
    }
}

// get a group of tiles as vector
impl<T: Clone> Grid<T> {
    pub fn corner_locations(&self) -> Vec<GridLocation> {
        let end_of_line = (self.grid_side_length - 1) as i32;
        vec![
            GridLocation::new(0, 0),
            GridLocation::new(0, end_of_line),
            GridLocation::new(end_of_line, 0),
            GridLocation::new(end_of_line, end_of_line),
        ]
    }

    pub fn edges_without_corners_locations(&self) -> Vec<GridLocation> {
        let end_of_line = (self.grid_side_length - 1) as i32;
        let mut edge_vector = vec![];
        for i in 1..end_of_line {
            edge_vector.push(GridLocation { row: 0, col: i }); //upper line
            edge_vector.push(GridLocation {
                row: end_of_line,
                col: i,
            }); //bottom line
            edge_vector.push(GridLocation { row: i, col: 0 }); //left line
            edge_vector.push(GridLocation {
                row: i,
                col: end_of_line,
            }); //right line
        }
        edge_vector
    }

    pub fn all_locations_no_edges(&self) -> Vec<GridLocation> {
        let end_of_line = (self.grid_side_length - 1) as i32;
        let mut edge_vector = vec![];
        for col in 1..end_of_line {
            for row in 1..end_of_line {
                edge_vector.push(GridLocation { row, col });
            }
        }
        edge_vector
    }
}

//basics
impl<T: Clone> Grid<T> {
    /// initializes to None
    pub fn new(grid_side_length: u8) -> Self {
        let mut grid: Vec<Option<T>> = vec![];
        for _ in 0..(grid_side_length * grid_side_length) {
            grid.push(None);
        }
        Self {
            grid_side_length,
            grid,
        }
    }

    pub fn get_side_length(&self) -> &u8 {
        &self.grid_side_length
    }

    pub fn get(&self, location: &GridLocation) 
    -> Result<Option<&T>, error_handler::GridError> 
    {
        if self.valid_index(location) {
            let location_index = self.location_to_index(location);
            match self.grid.get(location_index){
                None => Ok(None),
                Some(cell_value) => Ok(cell_value.as_ref())
            }
        } else {
            Err(error_handler::GridError::InvalidIndex(*location))
        }
    }

    pub fn get_mut(&mut self, location: &GridLocation)
    -> Result<Option<&mut T>, error_handler::GridError> 
    {
        if self.valid_index(location) {
            let location_index = self.location_to_index(location);
            match self.grid.get_mut(location_index){
                None => Ok(None),
                Some(cell_value) => Ok(cell_value.as_mut())
            }
        } else {
            Err(error_handler::GridError::InvalidIndex(*location))
        }
    }

    /// throws an error if the index is invalid
    pub fn set(&mut self, location: &GridLocation, value: T) -> Result<(), error_handler::GridError> {
        if self.valid_index(location) {
            self.grid[location.to_index(self.grid_side_length)] = Some(value);
            Ok(())
        }else{
            Err(error_handler::GridError::InvalidIndex(*location))
        }
    }

    /// returns an option with the previous value
    pub fn set_and_get_former(&mut self, location: &GridLocation, value: T) 
    -> Result<Option<T>,  error_handler::GridError>
    {
        if self.valid_index(location) {
            let former = self.grid[location.to_index(self.grid_side_length)].clone();
            self.set(location, value)?;
            Ok(former)
        } else {
            Ok(None)
        }
    }

    /// returns an option with the previous value
    pub fn set_none_get_former(&mut self, location: &GridLocation) 
    -> Result<Option<T>, error_handler::GridError> 
    {
        if self.valid_index(location) {
            let former = self.grid[location.to_index(self.grid_side_length)].clone();
            self.grid[location.to_index(self.grid_side_length)] = None;
            Ok(former)
        } else {
            Err(error_handler::GridError::InvalidIndex(*location))
        }
    }

    pub fn swap_by_location(&mut self, first: &GridLocation, second: &GridLocation) 
    -> Result<(), error_handler::GridError> 
    {
        if !self.valid_index(first){
            Err(error_handler::GridError::InvalidIndex(*first)) 
        }else if !self.valid_index(second){
            Err(error_handler::GridError::InvalidIndex(*second)) 
        }else{
            let first_location_index = self.location_to_index(first);
            let second_location_index = self.location_to_index(second);
            self.grid.swap(first_location_index, second_location_index);
            Ok(())
        }
    }

    pub fn valid_index(&self, location: &GridLocation) -> bool {
        location.col >= 0
            && location.row >= 0
            && location.col < self.grid_side_length as i32
            && location.row < self.grid_side_length as i32
    }

    pub fn location_from_index(&self, index: usize) -> GridLocation {
        GridLocation::from_index(index as u8, self.grid_side_length)
    }

    pub fn location_to_index(&self, location: &GridLocation) -> usize {
        location.to_index(self.grid_side_length)
    }
}

//iterators
impl<T: Clone> Grid<T> {
    /// returns occupied (initialized) cells' references only
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = (GridLocation, &T)> + '_ {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, optional_value)| optional_value.is_some())
            .map(|(index, optional_value)| {
                (
                    self.location_from_index(index),
                    optional_value.as_ref().unwrap(),
                )
            })
    }

    /// returns occupied (initialized) cells' references only
    pub fn iter_mut(&mut self) -> impl DoubleEndedIterator<Item = (GridLocation, &mut T)> + '_ {
        self.grid
            .iter_mut()
            .enumerate()
            .filter(|(_, optional_value)| optional_value.is_some())
            .map(|(index, optional_value)| {
                (
                    GridLocation::from_index(index as u8, self.grid_side_length),
                    optional_value.as_mut().unwrap(),
                )
            })
    }
}

impl<T: PartialEq + Eq + Clone> PartialEq for Grid<T> {
    fn eq(&self, other: &Self) -> bool {
        let mut all_cells_are_equal = true;
        let self_iter = self.iter();
        let other_iter = other.iter();
        for ((_, self_value), (_, other_value)) in self_iter.zip(other_iter) {
            if self_value != other_value {
                all_cells_are_equal = false;
                break;
            }
        }
        all_cells_are_equal
    }
}
impl<T: PartialEq + Eq + Clone> Eq for Grid<T> {}