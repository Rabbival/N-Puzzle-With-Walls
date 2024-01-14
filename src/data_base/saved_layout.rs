use crate::prelude::*;

#[derive(Debug)]
pub struct SavedLayout{
	pub board_propes: BoardProperties,
	pub wall_locations: Vec<GridLocation>
}