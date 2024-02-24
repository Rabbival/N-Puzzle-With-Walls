use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DomainBoard{
	pub board_propes: BoardProperties,
	pub wall_locations: Vec<GridLocation>
}