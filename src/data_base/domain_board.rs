use serde_json::json;
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DomainBoard{
	pub board_props: BoardProperties,
	pub wall_locations: Vec<GridLocation>
}

impl DomainBoard {
	fn to_json(&self) -> serde_json::Value {
		json!({
			"board_props": json!(self.board_props),
			"wall_locations": json!(self.wall_locations),
		})
	}
}