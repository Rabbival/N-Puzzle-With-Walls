use crate::prelude::*;

#[derive(Debug)]
pub struct GridTravellingTracker{
    /// keeps track of visited locations
    cells_locations_with_added_mark: Vec<GridTravellingCell>,
    /// locations not yet visited
    locations_to_visit: Vec<GridLocation>
}

impl GridTravellingTracker{
	
}

#[derive(Debug)]
struct GridTravellingCell{
    location: GridLocation,
    added_to_locations_to_visit: bool
}