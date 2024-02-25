use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainBoard{
	pub board_props: BoardProperties,
	pub wall_locations: Vec<GridLocation>
}

impl fmt::Display for DomainBoard{
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
		let mut wall_locations_string = String::from("[");
		for item in &self.wall_locations {
			wall_locations_string += &(String::from(" ") + &item.to_string());
		}
		wall_locations_string += " ]";

		fmt.write_str("DomainBoard(board_props: ")?;
		fmt.write_str(&self.board_props.to_string())?;
		fmt.write_str(", wall_locations:")?;
		fmt.write_str(&wall_locations_string)?;
		fmt.write_str(")")?;
		Ok(())
	}
}