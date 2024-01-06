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
						self.nodes.insert(node, Some(parent));
						// if the parent was a leaf up to this point,
						// remove it from the list of leaves
						util_functions::remove_by_value(
							&parent, 
							&mut self.leaves
						);

						// TODO: make previous function optional-
						// only call it if the parent was a leaf,
						// if not simply add one to its children counter

						true
					}else{
						false
					}
				}
			}
		}
	}

	/// returns true if node was removed successfully
	/// doesn't remove if it's not a leaf
	pub fn remove(&mut self, node_to_remove: GridLocation)-> bool{
		let index_of_node_to_remove = util_functions::item_to_index(
			&node_to_remove, 
			&mut self.leaves
		);
		if index_of_node_to_remove.is_none(){
			return false;
		}
		let leaf_to_remove = self.leaves
			.remove(index_of_node_to_remove.unwrap());
		let optional_parent_node 
			= self.nodes.get(&leaf_to_remove).unwrap();
		// if we didn't remove the root
		if let Some(parent) = optional_parent_node{

			// TODO: take one off from the node's children counter 
			// if it turns to zero, add it to the leaves

		}

	}
}