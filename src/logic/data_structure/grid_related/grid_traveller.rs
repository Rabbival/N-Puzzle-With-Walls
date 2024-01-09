use std::collections::VecDeque;

use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Debug)]
/// travels in BFS
pub struct GridTraveller<'a, T: Clone>{
	grid: &'a Grid<T>,
    /// keeps track of added locations,
	/// this is important because it could be that a place was added and visited
	/// and thus is no longer in locations_to_visit but shouldn't be added
    pub cells_locations_with_added_mark: HashMap<GridLocation, AddedToVisitPlan>,
    /// locations not yet visited
    pub locations_to_visit: VecDeque<GridLocation>,
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
			locations_to_visit
		}
	}

	/// checks if the location was added to the list of places to search.
	fn added_mark(&mut self, location: &GridLocation) -> Option<&mut bool>{
		Some(&mut self.cells_locations_with_added_mark.get_mut(location)?.0)
	}
}

/// BFS iterator, returns None if location wasn't found
/// or if there are no more locations to iterate over
impl<'a, T: Clone> Iterator for GridTraveller<'a, T>{
    type Item = LocationAndUnaddedNeighbors;

    fn next(&mut self) -> Option<Self::Item> {
		match self.locations_to_visit.pop_front(){
			None => None,
			Some(next_tile_to_visit) =>{
				let next_tile_neighbors 
					= self.grid.get_all_direct_neighbor_locations(&next_tile_to_visit);
				let new_locations_to_visit : VecDeque<GridLocation>
					= next_tile_neighbors
						.values()
						//only add the ones not yet visited
						.filter(|next_tile_neighbor_location|{
							match self.added_mark(next_tile_neighbor_location){
								// could be that the tile wasn't found,
								// but we can't return None from an iterator's closure
								// so we'll skip it
								None => {false},
								Some(added_mark) => ! *added_mark
							}
						})
						.copied()
						.collect();
				for new_location in new_locations_to_visit.clone(){
					*self.added_mark(&new_location)? = true;
				}
				self.locations_to_visit.append(&mut new_locations_to_visit.clone());
				Some(LocationAndUnaddedNeighbors{
					just_visited_location: next_tile_to_visit,
					just_added_neighbors: Vec::from(new_locations_to_visit)
				})
			}
		}
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AddedToVisitPlan(bool);

#[derive(Debug, Clone)]
pub struct LocationAndUnaddedNeighbors{
	pub just_visited_location: GridLocation,
	pub just_added_neighbors: Vec<GridLocation>
}