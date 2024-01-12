use crate::prelude::*;
use std::fmt;

#[derive(Clone, Debug)]
pub struct GridTreeNode {
	pub parent_location: Option<GridLocation>,
	pub depth: u8,
    pub children_counter: u8
}

impl GridTreeNode{
	pub fn new(parent_location: Option<GridLocation>, depth: u8)-> Self{
		Self { 
			parent_location, 
			depth,
			children_counter: 0 
		}
	}
}

impl fmt::Display for GridTreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let parent_loc_string = match self.parent_location{
			Some(loc) => loc.to_string(),
			None => String::from("None")
		};
        write!(f, "(parent: {},  depth: {})", 
			parent_loc_string, 
			self.depth
		)
    }
}