use crate::{prelude::*, logic::data_structure::util_functions};

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
	

	/// will make the parent node a leaf if it has no children left
	/// returns the removed leaf, or None if there was a problem and the iter should stop
	fn remove_random(&mut self)-> Option<GridLocation>{
		if self.leaves.is_empty(){
			return None;
		}
		let random_index_to_remove = util_functions::random_index(&self.leaves);
		let removed_leaf = self.leaves
			.remove(random_index_to_remove);
		let optional_parent_location 
			= self.nodes.get(&removed_leaf).unwrap().parent_location;
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
						let new_top_priority_leaf;
						match parent_node.depth {
							// if the paren't depth is 0 or 1, we don't want to remove it
							// just to be on the safe side
							0 | 1 => new_top_priority_leaf = None,
							_ => {
								new_top_priority_leaf = Some(parent_location);
								self.leaves.push(parent_location);
							}
						};
						self.top_priority_leaf = new_top_priority_leaf;
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
		match self.top_priority_leaf{
			Some(top_priority_location) => Some(top_priority_location),
			// if there's no top priority leaf, return a random leaf
			None => {
				Some(self.remove_random()?)
			}
		}
    }
}

impl Default for GridTree {
    fn default() -> Self {
         Self::new()
     }
}



#[derive(Clone, Debug)]
struct GridTreeNode {
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