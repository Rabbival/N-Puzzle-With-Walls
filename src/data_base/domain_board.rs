use json::JsonValue;
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct DomainBoard{
	pub board_props: BoardProperties,
	pub wall_locations: Vec<GridLocation>
}

impl Into<JsonValue> for DomainBoard {
	fn into(self) -> JsonValue {
		let mut json_obj = JsonValue::new_object();
		json_obj["board_props"] = self.board_props.into();
		json_obj["wall_locations"] = self.wall_locations.into();
		json_obj
	}
}