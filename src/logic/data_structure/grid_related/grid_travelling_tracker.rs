use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Debug)]
/// travels in DFS
pub struct GridTraveller{
    /// keeps track of added location,
	/// this is important because it could be that a place was added and visited
	/// and thus is no longer in locations_to_visit but shouldn't be added
    pub cells_locations_with_added_mark: Vec<GridTravellerCell>,
    /// locations not yet visited
    pub locations_to_visit: Vec<GridLocation>,
	/// how many cells visited up to that point
	pub cells_visited_counter: u32
}

impl GridTraveller{
	/// builds a travelling tracker from a grid 
	/// that starts at a random initialzed cell
	pub fn from_grid<T: Clone>(grid: &Grid<T>) -> Self{
		let cells_locations_with_added_mark: Vec<GridTravellerCell>=
			grid
			.iter()
			.map(|(location, _)| {
				GridTravellerCell::from_location(&location)
			})
			.collect();
		let mut first_grid_travelling_cell 
			= util_functions::random_value(&cells_locations_with_added_mark);
		let locations_to_visit 
			= vec![first_grid_travelling_cell.location];
		first_grid_travelling_cell.added_to_locations_to_visit = true;

		Self { 
			cells_locations_with_added_mark, 
			locations_to_visit, 
			cells_visited_counter: 0 
		}
	}

	pub fn next_cell_location<T: Clone>(&mut self, grid: &Grid<T>)-> GridLocation{
		let next_tile_to_check 
			= self.locations_to_visit.pop().unwrap();
		let next_tile_neighbors 
			= grid.get_all_direct_neighbor_locations(&next_tile_to_check);
		let mut new_locations_to_visit : Vec<GridLocation>
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
		let optional_location_index = 
			self.cells_locations_with_added_mark
			.iter()
			.position(|grid_travelling_cell| {
				grid_travelling_cell.location == *location
			});

		Some(
			&mut self.cells_locations_with_added_mark
				.get_mut(optional_location_index?)
				.unwrap().added_to_locations_to_visit
		)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct GridTravellerCell{
    pub location: GridLocation,
    pub added_to_locations_to_visit: bool
}

impl GridTravellerCell{
	pub fn from_location(location: &GridLocation) -> Self{
		Self { 
			location: *location, 
			added_to_locations_to_visit: false 
		}
	}
}