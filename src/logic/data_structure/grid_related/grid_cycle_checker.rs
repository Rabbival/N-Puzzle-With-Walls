use crate::prelude::*;
use crate::prelude::GridTreeError::ParentNotFound;

pub struct GridCycleChecker<'a, T: Clone>{
    grid_traveller: GridTraveller<'a, T>,
    grid_tree: GridTree,
    cycle_markers: HashMap<GridLocation, InCycle>,
    locations_not_in_cycle_counter: u32
}

//constructors
impl<'a, T: Clone> GridCycleChecker<'a, T>{
    pub fn new(grid: &'a Grid<T>) -> Self {
        let grid_traveller = GridTraveller::from_grid(grid, GridTravellerType::DFS);
        Self{
            grid_traveller: grid_traveller.clone(),
            grid_tree: GridTree::from_root(grid_traveller.locations_to_visit[0]),
            cycle_markers: HashMap::new(),
            locations_not_in_cycle_counter: 0
        }
    }
}

impl<'a, T: Clone> GridCycleChecker<'a, T>{
    pub fn all_nodes_in_cycles(&mut self, grid: &Grid<T>)
        -> Result<bool, DataStructError<GridLocation>>
    {
        loop {
            let optional_location_and_newly_added_neighbors = self.grid_traveller.next();
            match optional_location_and_newly_added_neighbors {
                Some(location_and_newly_added_neighbors) => {
                    self.add_visited_location_and_newly_added_neighbors(&location_and_newly_added_neighbors)?;
                    let cycle_tracking_progress_result = self.progress_cycle_tracking(&location_and_newly_added_neighbors, grid);
                    if let Err(error_from_progressing) = cycle_tracking_progress_result {
                        return match error_from_progressing {
                            DataStructError::GridTreeError(ParentNotFound) => Ok(false),
                            _ => Err(error_from_progressing)
                        };
                    }

                    // info!("not in cycle: {:?}", self.locations_not_in_cycle_counter);

                },
                None => break
            }
        }
        Ok(self.locations_not_in_cycle_counter == 0)
    }

    fn add_visited_location_and_newly_added_neighbors(
        &mut self,
        location_and_unadded_neighbors: &LocationAndUnaddedNeighbors
    ) -> Result<(), DataStructError<GridLocation>>
    {
        let just_visited_location = location_and_unadded_neighbors.just_visited_location;
        let just_added_neighbors = location_and_unadded_neighbors.just_added_neighbors.clone();

        // don't want to override already found neighbors (inserted when spotted)
        if !self.cycle_markers.contains_key(&just_visited_location){
            self.cycle_markers.insert(just_visited_location, InCycle(false));
            self.locations_not_in_cycle_counter += 1;
        }

        for neighbor in just_added_neighbors {
            if let Err(tree_error) = self.grid_tree.insert_leaf(neighbor, Some(just_visited_location)) {
                return Err(DataStructError::GridTreeError(tree_error));
            }
        }
        Ok(())
    }

    fn progress_cycle_tracking(
        &mut self,
        location_and_unadded_neighbors: &LocationAndUnaddedNeighbors,
        grid: &Grid<T>
    ) -> Result<(), DataStructError<GridLocation>>
    {
        let just_visited_location = location_and_unadded_neighbors.just_visited_location;

        let mut already_added_neighbors =
            self.find_already_added_neighbors(location_and_unadded_neighbors, grid)?;
        // the parent doesn't count as closing a cycle
        let optional_just_visited_location_parent =
            self.grid_tree.get_grid_tree_node(&just_visited_location).unwrap().parent_location;
        if let Some(parent_location) = optional_just_visited_location_parent {
            remove_by_value(
                &parent_location,
                &mut already_added_neighbors
            );
        }



        // info!("just visited: {:?}", just_visited_location);
        // info!("spotted already added neighbors: {:?}", already_added_neighbors);



        // if we got to a place already added, we closed a cycle
        for already_added_neighbor in already_added_neighbors{
            if self.cycle_markers.contains_key(&already_added_neighbor){
                continue;
            }
            self.cycle_markers.insert(already_added_neighbor, InCycle(true));


            // info!("marked as part of cycle {:?}", already_added_neighbor);


            self.declare_locations_as_part_of_cycle_by_parent_of(already_added_neighbor, just_visited_location)?;
        }
        Ok(())
    }

    fn find_already_added_neighbors(
        &self,
        location_and_unadded_neighbors: &LocationAndUnaddedNeighbors,
        grid: &Grid<T>
    ) -> Result<Vec<GridLocation>,DataStructError<GridLocation>> {
        let neighbors_and_directions_of_last_visited_location =
            grid.get_all_initialized_neighbor_locations(&location_and_unadded_neighbors.just_visited_location);
        let neighbors_of_just_visited_location : Vec<GridLocation> =
            neighbors_and_directions_of_last_visited_location.values().copied().collect();
        let mut unadded_neighbors_of_last_visited_location = neighbors_of_just_visited_location;
        for just_added_neighbor in location_and_unadded_neighbors.just_added_neighbors.clone(){
            let optional_just_added_location = remove_by_value(
                &just_added_neighbor,
                &mut unadded_neighbors_of_last_visited_location
            );
            if optional_just_added_location.is_none(){
                return Err(DataStructError::ItemNotFound(just_added_neighbor))
            }
        }
        Ok(unadded_neighbors_of_last_visited_location)
    }

    fn declare_locations_as_part_of_cycle_by_parent_of(
        &mut self,
        already_added_neighbor: GridLocation,
        just_visited_location: GridLocation
    ) -> Result<(), DataStructError<GridLocation>>
    {
        self.mark_location_as_part_of_cycle_if_it_wasnt_marked_so(&just_visited_location)?;
        let optional_already_added_neighbor_parent =
            self.grid_tree.get_grid_tree_node(&already_added_neighbor).unwrap().parent_location;
        if let Some(parent_of_already_added_neighbor) = optional_already_added_neighbor_parent{
            let mut optional_parent_of_last_tracked =
                self.grid_tree.get_grid_tree_node(&just_visited_location).unwrap().parent_location;
            while let Some(track_back_parent) = optional_parent_of_last_tracked{
                self.mark_location_as_part_of_cycle_if_it_wasnt_marked_so(&track_back_parent)?;
                if track_back_parent == parent_of_already_added_neighbor {
                    break;
                }


                // info!("parent of last tracked location: {:?}", optional_parent_of_last_tracked);


                optional_parent_of_last_tracked =
                    self.grid_tree.get_grid_tree_node(&track_back_parent).unwrap().parent_location;
            }
            if optional_parent_of_last_tracked.is_none() {
                return Err(DataStructError::GridTreeError(ParentNotFound));
            }

            self.mark_location_as_part_of_cycle_if_it_wasnt_marked_so(&parent_of_already_added_neighbor)?;
        }
        Ok(())
    }

    fn mark_location_as_part_of_cycle_if_it_wasnt_marked_so(
        &mut self,
        location_to_mark: &GridLocation
    ) -> Result<(), DataStructError<GridLocation>>
    {
        let cycle_markers = &mut self.cycle_markers;
        let locations_not_in_cycle_counter = &mut self.locations_not_in_cycle_counter;

        match cycle_markers.get_mut(location_to_mark){
            Some(marked_as_part_of_cycle) => {
                if ! marked_as_part_of_cycle.0 {
                    marked_as_part_of_cycle.0 = true;
                    *locations_not_in_cycle_counter -= 1;


                    // info!("marked as part of cycle {:?}", location_to_mark);


                }
                Ok(())
            },
            None => {
                Err(DataStructError::ItemNotFound(*location_to_mark))
            }
        }
    }
}


#[derive(Debug)]
struct InCycle(pub bool);