use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Component, Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainBoard{
	pub board_props: BoardProperties,
	pub wall_locations: Vec<GridLocation>
}

impl DomainBoard{
	pub fn to_string_for_button(&self) -> String {
		let props = self.board_props;
		
		let mut string_for_button = String::from("Dif: ");
		string_for_button += &props.board_difficulty.to_string();
		string_for_button += ", Gen: ";
		string_for_button += &props.generation_method.to_string();

		string_for_button
	}

	fn stringify_wall_locations(&self) -> String {
		let mut wall_locations_string = String::from("[");
		for item in &self.wall_locations {
			wall_locations_string += &(String::from(" ") + &item.to_string());
		}
		wall_locations_string += " ]";
		wall_locations_string
	}
}

impl fmt::Display for DomainBoard{
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
		fmt.write_str("board_props: ")?;
		fmt.write_str(&self.board_props.to_string())?;
		fmt.write_str(", wall_locations:")?;
		fmt.write_str(&self.stringify_wall_locations())?;
		fmt.write_str(")")?;
		Ok(())
	}
}