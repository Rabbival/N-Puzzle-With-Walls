use std::collections::VecDeque;

use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Debug)]
/// travels in BFS
pub struct GridTraveller<'a, T: Clone>{
	grid: &'a Grid<T>,
    /// keeps track of added location,
	/// this is important because it could be that a place was added and visited
	/// and thus is no longer in locations_to_visit but shouldn't be added
    pub cells_locations_with_added_mark: HashMap<GridLocation, AddedToVisitPlan>,
    /// locations not yet visited
    pub locations_to_visit: VecDeque<GridLocation>,
	/// how many cells visited up to that point
	pub cells_visited_counter: u32
}

impl<'a, T: Clone> GridTraveller<'a, T>{
	/// builds a travelling tracker from a grid 
	/// that starts at a random initialzed cell
	pub fn from_grid(grid: &'a Grid<T>) -> Self{
		let mut cells_locations_with_added_mark: HashMap<GridLocation, AddedToVisitPlan>=
			grid
			.iter()
			.map(|(location, _)| {
				(location, AddedToVisitPlan(false))
			})
			.collect();
		let first_location 
			= *util_functions::random_value(
				&cells_locations_with_added_mark
					.keys()
					.collect()
			);
		let first_added_mark 
			= cells_locations_with_added_mark
				.get_mut(&first_location).unwrap();
		let locations_to_visit
			= VecDeque::from([first_location]);
		first_added_mark.0 = true;

		Self { 
			grid,
			cells_locations_with_added_mark, 
			locations_to_visit, 
			cells_visited_counter: 0 
		}
	}

	/// travel in BFS
	pub fn next_cell_location(&mut self)-> GridLocation{
		let next_tile_to_check 
			= self.locations_to_visit.pop_front().unwrap();
		let next_tile_neighbors 
			= self.grid.get_all_direct_neighbor_locations(&next_tile_to_check);
		let mut new_locations_to_visit : VecDeque<GridLocation>
			= next_tile_neighbors
				.values()
				//only add the ones not yet visited
				.filter(|next_tile_neighbor_location|{
					! *self.added_mark(next_tile_neighbor_location).unwrap()
				})
				.copied()
				.collect();
		for new_location in new_locations_to_visit.clone(){
			*self.added_mark(&new_location).unwrap() = true;
		}
		self.locations_to_visit.append(&mut new_locations_to_visit);
		next_tile_to_check
	}

	/// checks if the location was added to the list of places to search.
	fn added_mark(&mut self, location: &GridLocation) -> Option<&mut bool>{
		Some(&mut self.cells_locations_with_added_mark.get_mut(location)?.0)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct AddedToVisitPlan(bool);