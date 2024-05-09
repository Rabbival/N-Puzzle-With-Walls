use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Component, Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainBoard{
	pub board_props: BoardProperties,
	pub grid: Grid<Tile>
}

impl DomainBoard{
	pub fn to_string_for_button(&self) -> String {
		let props = self.board_props;
		
		let mut string_for_button = String::from("Generated: ");
		string_for_button += &props.generation_method.to_string();

		string_for_button
	}
}

impl fmt::Display for DomainBoard{
	fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
		fmt.write_str("board_props: ")?;
		fmt.write_str(&self.board_props.to_string())?;
		fmt.write_str(&format!(", grid: {:?}", self.grid.clone()))?;
		fmt.write_str(")")?;
		Ok(())
	}
}