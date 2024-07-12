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
	pub fn has_valid_counters_and_size(&self) -> bool{
		if self.grid.get_side_length() != self.board_props.size.to_grid_side_length(){
			return false;
		}
		let specified_walls_count = self.board_props.wall_count;
		let specified_empties_count = self.board_props.empty_count;
		let mut walls_counter = 0;
		let mut empties_counter = 0;
		for tile_type in self.grid.iter().map(|(_, tile)| tile.tile_type){
			match tile_type{
				TileType::Empty => {
					empties_counter += 1;
					if empties_counter > specified_empties_count{
						return false;
					}
				},
				TileType::Wall => {
					walls_counter += 1;
					if walls_counter > specified_walls_count{
						return false;
					}
				},
				_ => {}
			}
		}
		true
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