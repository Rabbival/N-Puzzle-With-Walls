use crate::logic::data_structure::util_functions;
use crate::output::error_handler;
use crate::prelude::*;

pub struct GridCycleChecker<'a, T: Clone>{
    grid_traveller: GridTraveller<'a, T>,
    grid_tree: GridTree,
    locations_visited_in_order: Vec<GridLocation>,
    cycle_markers: HashMap<GridLocation, InCycle>,
    locations_not_in_cycle_counter: u32
}

//constructors
impl<'a, T: Clone> GridCycleChecker<'a, T>{
    pub fn new(grid: &'a Grid<T>) -> Self {
        let grid_traveller = GridTraveller::from_grid(grid, GridTravellerType::DFS);
        Self{
            grid_traveller,
            grid_tree: GridTree::from_root(grid_traveller.locations_to_visit[0]),
            locations_visited_in_order: Vec::new(),
            cycle_markers: HashMap::new(),
            locations_not_in_cycle_counter: 0
        }
    }
}

impl<'a, T: Clone> GridCycleChecker<'a, T>{
    pub fn all_nodes_in_cycles(&mut self, grid: &Grid<T>)
        -> Result<bool, error_handler::DataStructError<GridLocation>>
    {
        for location_and_newly_added_neighbors in self.grid_traveller {
            self.add_visited_location_and_newly_added_neighbors(location_and_newly_added_neighbors)?;
        }
        Ok(self.locations_not_in_cycle_counter == 0)
    }
}



//TODO: keep workin from here

impl<T: Clone> Grid<T>{


    fn progress_traveller_and_update_cycles(
        &self,
        location_and_unadded_neighbors: LocationAndUnaddedNeighbors,
        cycle_markers: &mut HashMap<GridLocation, InCycle>,
        locations_visited_in_order: &mut Vec<GridLocation>,
        grid_tree: &mut GridTree,
        traveller_for_visited_updates: &mut GridTraveller<T>,
        locations_not_in_cycle_counter: &mut u32
    ) -> Result<(), error_handler::DataStructError<GridLocation>>
    {
        let just_visited_location = location_and_unadded_neighbors.just_visited_location;
        let just_added_neighbors = location_and_unadded_neighbors.just_added_neighbors.clone();

        //ADD
        locations_visited_in_order.push(just_visited_location);
        cycle_markers.insert(just_visited_location, InCycle(false));
        *locations_not_in_cycle_counter += 1;
        for neighbor in just_added_neighbors {
            if let Err(tree_error) = grid_tree.insert_leaf(neighbor, Some(just_visited_location)){
                return Err(DataStructError::GridTreeError(tree_error));
            }
        }


        info!("locations not in cycle: {:?}", locations_visited_in_order);


        let mut already_added_neighbors =
            self.find_already_added_neighbors(&location_and_unadded_neighbors)?;

        // the parent doesn't count as closing a cycle
        let optional_just_visited_location_parent =
            grid_tree.get_grid_tree_node(&just_visited_location).unwrap().parent_location;
        if let Some(parent_location) = optional_just_visited_location_parent {
            util_functions::remove_by_value(
                &parent_location,
                &mut already_added_neighbors
            );
        }

        // if we got to a place already added, we closed a cycle
        for already_added_neighbor in already_added_neighbors{
            traveller_for_visited_updates.mark_as_visited(&already_added_neighbor);
            declare_locations_as_part_of_cycle_up_to_parent_including(
                already_added_neighbor,
                cycle_markers,
                locations_visited_in_order,
                grid_tree,
                locations_not_in_cycle_counter
            );
        }
        Ok(())
    }

    fn find_already_added_neighbors(
        &self,
        location_and_unadded_neighbors: &LocationAndUnaddedNeighbors
    ) -> Result<Vec<GridLocation>,error_handler::DataStructError<GridLocation>> {
        let neighbors_and_directions_of_last_visited_location =
            self.get_all_occupied_neighbor_locations(&location_and_unadded_neighbors.just_visited_location);
        let neighbors_of_just_visited_location : Vec<GridLocation> =
            neighbors_and_directions_of_last_visited_location.values()
                .map(|neighbor_location| {
                    *neighbor_location
                }).collect();
        let mut unadded_neighbors_of_last_visited_location = neighbors_of_just_visited_location;
        for just_added_neighbor in location_and_unadded_neighbors.just_added_neighbors.clone(){
            let optional_just_added_location = util_functions::remove_by_value(
                &just_added_neighbor,
                &mut unadded_neighbors_of_last_visited_location
            );
            if optional_just_added_location.is_none(){
                return Err(error_handler::DataStructError::ItemNotFound(just_added_neighbor))
            }
        }
        Ok(unadded_neighbors_of_last_visited_location)
    }
}

fn declare_locations_as_part_of_cycle_up_to_parent_including(
    already_added_neighbor: GridLocation,
    cycle_markers: &mut HashMap<GridLocation, InCycle>,
    locations_not_in_cycles: &mut Vec<GridLocation>,
    grid_tree: &mut GridTree,
    locations_not_in_cycle_counter: &mut u32
){
    let already_added_neighbor_optional_parent =
        grid_tree.get_grid_tree_node(&already_added_neighbor).unwrap().parent_location;
    if let Some(parent_location) = already_added_neighbor_optional_parent{


        info!("ordered to remove: {:?}", parent_location);


        retrace_declare_up_to_including(
            locations_not_in_cycles,
            &parent_location
        );
    }
}

fn retrace_declare_up_to_including(
    locations_visited_in_order: &mut Vec<GridLocation>,
    parent_location: &GridLocation
){

    // TODO: keep popping and checking parents, stop removing when parent is correct, then remove parent with util fn
    //
    // let
    // while
    // ! locations_travelled.is_empty() &&
    //
    // {}
}

struct InCycle(pub bool);