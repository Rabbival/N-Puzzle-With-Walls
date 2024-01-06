use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Clone, Debug)]
/// a tree that tracks locations in the grid,
/// inteneded to be used as MST
pub struct GridTree {
	/// sorted as <value, parent location>
	nodes: HashMap<GridLocation, Option<GridLocation>>,
    leaves: Vec<GridLocation>
}

// constructors
impl GridTree{
	pub fn new() -> Self{
		Self { 
			nodes: HashMap::new(), 
			leaves: vec![] 
		}
	}

	pub fn from_root(root_location: GridLocation) -> Self{
		let mut nodes = HashMap::new();
		nodes.insert(root_location, None);
		Self { 
			nodes, 
			leaves: vec![root_location] 
		}
	}
}

impl GridTree{
	pub fn is_empty(&self)-> bool{
		self.nodes.is_empty()
	}

	/// returns true if leaf was inserted successfully
	/// doesn't insert if parent wasn't found or node already exists in the tree
	pub fn insert(&mut self, node: GridLocation, optional_parent: Option<GridLocation>)-> bool{
		match self.nodes.get(&node){
			Some(_) => false,
			None => {
				// if the tree is empty, the new node must have no parent
				if self.nodes.is_empty(){
					if optional_parent.is_none(){
						self.nodes.insert(node, None);
						self.leaves=vec![node];
						true
					}else{
						false
					}
				}else{
					if let Some(parent) = optional_parent{
						self.leaves.push(node);
						self.nodes.insert(node, Some(parent))
						// if the parent was a leaf up to this point,
						// remove it from the list of leaves
						util_functions::remove_by_value(
							&parent, 
							&mut self.leaves
						);
						true
					}else{
						false
					}
				}
			}
		}
	}

	/// returns true if node was removed successfully
	/// doesn't insert if parent wasn't found or child already exists in the tree
	pub fn remove_leaf(&mut self, parent: GridLocation, child: GridLocation)-> bool{
		match self.nodes.get(&child){
			Some(_) => false,
			None => {
				let optional_children = self.nodes.get_mut(&parent);
				match optional_children{
					//parent doesn't exist
					None => false,
					Some(children) => {
						children.push(child);
						self.leaves.push(child);
						return true;
					}
				}
			}
		}
	}
}