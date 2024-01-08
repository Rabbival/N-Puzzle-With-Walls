use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Clone, Debug)]
/// a tree that tracks locations in the grid,
/// inteneded to be used as MST
pub struct GridTree {
	/// sorted as <node, node_properties>
	nodes: HashMap<GridLocation, GridTreeNode>,
    leaves: Vec<GridLocation>,
	/// a leaf to be drawn out before the others
	top_prority_leaf: Option<GridLocation>
}

// constructors
impl GridTree{
	pub fn new() -> Self{
		Self { 
			nodes: HashMap::new(), 
			leaves: vec![] ,
			top_prority_leaf: None
		}
	}

	pub fn from_root(root_location: GridLocation) -> Self{
		let mut nodes = HashMap::new();
		nodes.insert(root_location, GridTreeNode::new(None, 0));
		Self { 
			nodes, 
			leaves: vec![root_location] ,
			top_prority_leaf: None
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
	/// doesn't remove if it's not a leaf
	/// returns true if node was removed successfully
	fn remove(&mut self, node_to_remove: GridLocation)-> bool{
		let index_of_node_to_remove = util_functions::item_to_index(
			&node_to_remove, 
			&self.leaves
		);
		if index_of_node_to_remove.is_none(){
			return false;
		}
		let removed_leaf = self.leaves
			.remove(index_of_node_to_remove.unwrap());
		let optional_parent_location 
			= self.nodes.get(&removed_leaf).unwrap().parent_location;
		// if we didn't remove the root
		if let Some(parent_location) = optional_parent_location{
			let optional_parent_node =
				self.nodes.get_mut(&parent_location);
			match optional_parent_node{
				None => return false,
				Some(parent_node) => {
					parent_node.children_counter -= 1;
					if parent_node.children_counter == 0{
						self.leaves.push(parent_location);
						let new_top_priority_leaf = match parent_node.depth{
							0 | 1 => None,
							_ => Some(parent_location)
						};
						self.top_prority_leaf = new_top_priority_leaf;
					}
				}
			}
		}
		true
	}
}

// impl Iterator for GridTree{
//     type Item = GridLocation;

//     fn next(&mut self) -> Option<Self::Item> {
			// if there's a top priority return it
			// if there's None return a random leaf
//     }
// }

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