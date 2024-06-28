use std::fmt;
use std::fmt::Formatter;
use serde::{Deserialize, Serialize};
use crate::prelude::*;

#[derive(Component, Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainBoard{
	pub board_props: BoardProperties,
	pub grid: Grid<Tile>
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