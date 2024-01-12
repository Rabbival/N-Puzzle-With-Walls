use crate::{prelude::*, logic::data_structure::util_functions};
use super::grid_tree_node::*;

const MINIMAL_DEPTH: u8 = 2;

#[derive(Clone, Debug)]
/// a tree that tracks locations in the grid,
/// inteneded to be used as MST
pub struct GridTree {
	/// sorted as <node, node_properties>
	nodes: HashMap<GridLocation, GridTreeNode>,
    leaves: Vec<GridLocation>,
	/// a leaf to be drawn out before the others
	top_priority_leaf: Option<GridLocation>
}

// constructors
impl GridTree{
	pub fn new() -> Self{
		Self { 
			nodes: HashMap::new(), 
			leaves: vec![] ,
			top_priority_leaf: None
		}
	}

	pub fn from_root(root_location: GridLocation) -> Self{
		let mut nodes = HashMap::new();
		nodes.insert(root_location, GridTreeNode::new(None, 0));
		Self { 
			nodes, 
			leaves: vec![root_location] ,
			top_priority_leaf: None
		}
	}
}

impl GridTree{
	pub fn is_empty(&self)-> bool{
		self.nodes.is_empty()
	}

	/// returns true if leaf was inserted successfully
	/// doesn't insert if parent wasn't found or node already exists in the tree
	pub fn insert(
		&mut self, 
		node: GridLocation, 
		optional_parent_location: Option<GridLocation>
	)-> bool
	{
		if self.nodes.get(&node).is_some(){
			false
		}else{
			// if the tree is empty, the new node must have no parent
			if self.nodes.is_empty(){
				if optional_parent_location.is_none(){
					self.nodes.insert(node, GridTreeNode::new(None, 0));
					self.leaves=vec![node];
					true
				}else{
					false
				}
			}else if let Some(parent_location) 
				= optional_parent_location
			{
				let optional_parent_node 
					= self.nodes.get_mut(&parent_location);
				match optional_parent_node{
					//if the parent doesn't exist the request is invalid
					None => false,
					Some(parent_node) =>{
						// if the parent was a leaf up to this point,
						// remove it from the list of leaves
						let parent_children_counter = 
							&mut parent_node.children_counter;
						if *parent_children_counter == 0{
							util_functions::remove_by_value(
								&parent_location, 
								&mut self.leaves
							);
						}
						*parent_children_counter += 1;

						let parent_depth = parent_node.depth;
						self.leaves.push(node);
						self.nodes.insert(
							node, 
							GridTreeNode::new(
								Some(parent_location),
								parent_depth + 1
							)
						);

						true
					}
				}
			}else{
				false
			}
		}
	}
	

	/// if leaf_to_remove is None, picks one randomly
	/// will make the parent node a leaf if it has no children left
	/// returns the removed leaf, or None if there was a problem and the iter should stop
	fn remove_leaf(&mut self, leaf_to_remove: Option<GridLocation>)-> Option<GridLocation>{
		if self.leaves.is_empty(){
			return None;
		}
		let mut removed_leaf = match leaf_to_remove{
			Some(leaf_location) => {
				util_functions::remove_by_value(
					&leaf_location, 
					&mut self.leaves
				);
				leaf_location
			},
			None => {
				let index_to_remove 
					= util_functions::random_index(&self.leaves);
				self.leaves.remove(index_to_remove)
			}
		};
		let mut leaf_props = self.nodes.get(&removed_leaf).unwrap();
		// can't remove a leaf if its too close to the root to keep connectivity
		while leaf_props.depth < MINIMAL_DEPTH {
			if self.leaves.is_empty(){
				return None;
			}else{
				let index_to_remove 
					= util_functions::random_index(&self.leaves);
				removed_leaf = self.leaves.remove(index_to_remove);
				leaf_props = self.nodes.get(&removed_leaf).unwrap();
			}
		}


		let optional_parent_location = leaf_props.parent_location;
		// if we didn't remove the root
		if let Some(parent_location) = optional_parent_location{
			let optional_parent_node =
				self.nodes.get_mut(&parent_location);
			match optional_parent_node{
				None => return None,
				Some(parent_node) => {
					parent_node.children_counter -= 1;
					// if the parent is a leaf, could be that it could be removed
					if parent_node.children_counter == 0{
						// if the paren't depth is 0 or 1, we don't want to remove it
						// just to be on the safe side
						if parent_node.depth > MINIMAL_DEPTH {
							self.top_priority_leaf = Some(parent_location);
							self.leaves.push(parent_location);
						}
					}
				}
			}
		}
		Some(removed_leaf)
	}
}

impl Iterator for GridTree{
    type Item = GridLocation;

    fn next(&mut self) -> Option<Self::Item> {


		self.print_leaves(true);


		match self.top_priority_leaf{
			Some(top_priority_location) => {
				self.top_priority_leaf = None;
				Some(self.remove_leaf(Some(top_priority_location))?)
			},
			// if there's no top priority leaf, return a random leaf
			None => {
				Some(self.remove_leaf(None)?)
			}
		}
    }
}

impl Default for GridTree {
    fn default() -> Self {
         Self::new()
     }
}

//debug
impl GridTree{
	fn print_leaves(&self, props_included: bool){
		let mut leaves_string = String::from("");
		for leaf in self.leaves.clone(){
			leaves_string += "\n";
			leaves_string += &leaf.to_string();	
			if props_included{
				let leaf_prop = self.nodes.get(&leaf);
				if let Some(props) = leaf_prop{
					leaves_string += &(String::from(" ") + &props.to_string());	
				}
			}		
		}
		info!("{}", leaves_string);
	}
}