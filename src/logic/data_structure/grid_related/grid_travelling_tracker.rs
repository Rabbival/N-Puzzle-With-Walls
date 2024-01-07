use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Debug)]
pub struct GridTravellingTracker{
    /// keeps track of visited locations
    pub cells_locations_with_added_mark: Vec<GridTravellingCell>,
    /// locations not yet visited
    pub locations_to_visit: Vec<GridLocation>,
	/// how many cells visited up to that point
	pub cells_visited_counter: u32
}

impl GridTravellingTracker{
	/// builds a travelling tracker from a grid 
	/// that starts at a random initialzed cell
	pub fn from_grid<T: Clone>(grid: &Grid<T>) -> Self{
		let cells_locations_with_added_mark: Vec<GridTravellingCell>=
			grid
			.iter()
			.map(|(location, _)| {
				GridTravellingCell::from_location(&location)
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
}

#[derive(Debug, Clone, Copy)]
pub struct GridTravellingCell{
    pub location: GridLocation,
    pub added_to_locations_to_visit: bool
}

impl GridTravellingCell{
	pub fn from_location(location: &GridLocation) -> Self{
		Self { 
			location: *location, 
			added_to_locations_to_visit: false 
		}
	}
}