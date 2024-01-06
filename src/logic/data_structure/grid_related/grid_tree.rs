use crate::{prelude::*, logic::data_structure::util_functions};

#[derive(Clone, Debug)]
/// a tree that tracks locations in the grid,
/// inteneded to be used as MST
pub struct GridTree {
	/// sorted as <value, parent location>
	nodes: HashMap<GridLocation, Option<GridTreeNode>>,
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
	pub fn insert(
		&mut self, 
		node: GridLocation, 
		optional_parent_location: Option<GridLocation>
	)-> bool
	{
		match self.nodes.get(&node){
			Some(_) => false,
			None => {
				// if the tree is empty, the new node must have no parent
				if self.nodes.is_empty(){
					if optional_parent_location.is_none(){
						self.nodes.insert(node, None);
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
    									&mut parent_node.as_mut().unwrap().children_counter;
    								if *parent_children_counter == 0{
    									util_functions::remove_by_value(
    										&parent_location, 
    										&mut self.leaves
    									);
    								}
    								*parent_children_counter += 1;

    								self.leaves.push(node);
    								self.nodes.insert(node, 
    									Some(GridTreeNode::new(parent_location))
    								);

    								true
    							}
    						}
    					}else{
    						false
    					}
			}
		}
	}

	/// returns true if node was removed successfully
	/// doesn't remove if it's not a leaf
	pub fn remove(&mut self, node_to_remove: GridLocation)-> bool{
		let index_of_node_to_remove = util_functions::item_to_index(
			&node_to_remove, 
			&self.leaves
		);
		if index_of_node_to_remove.is_none(){
			return false;
		}
		let leaf_to_remove = self.leaves
			.remove(index_of_node_to_remove.unwrap());
		let optional_parent_node 
			= self.nodes.get_mut(&leaf_to_remove).unwrap();
		// if we didn't remove the root
		if let Some(parent_node) = optional_parent_node{
			parent_node.children_counter -= 1;
			if parent_node.children_counter == 0{
				self.leaves.push(parent_node.value);
			}
		}
		true
	}
}

impl Default for GridTree {
    fn default() -> Self {
         Self::new()
     }
}



#[derive(Clone, Debug)]
struct GridTreeNode {
	pub value: GridLocation,
    pub children_counter: u8
}

impl GridTreeNode{
	pub fn new(location: GridLocation)-> Self{
		Self { 
			value: location, 
			children_counter: 0 
		}
	}
}