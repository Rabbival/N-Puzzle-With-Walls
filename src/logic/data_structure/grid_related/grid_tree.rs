use super::grid_tree_node::*;
use crate::{logic::data_structure::util_functions, prelude::*, output::error_handler};

const MINIMAL_DEPTH: u8 = 2;

#[derive(Clone, Debug)]
/// a tree that tracks locations in the grid,
/// inteneded to be used as MST
pub struct GridTree {
    /// sorted as <node, node_properties>
    nodes: HashMap<GridLocation, GridTreeNode>,
    leaves: Vec<GridLocation>,
    /// a leaf to be drawn out before the others
    top_priority_leaf: Option<GridLocation>,
}

// constructors
impl GridTree {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            leaves: vec![],
            top_priority_leaf: None,
        }
    }

    pub fn from_root(root_location: GridLocation) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(root_location, GridTreeNode::new(None, 0));
        Self {
            nodes,
            leaves: vec![root_location],
            top_priority_leaf: None,
        }
    }
}

impl GridTree {
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn insert_leaf(
        &mut self,
        node: GridLocation,
        optional_parent_location: Option<GridLocation>,
    ) -> Result<(), error_handler::GridTreeError> {
        if self.nodes.get(&node).is_some() {
            Err(error_handler::GridTreeError::NodeAlreadyExists)
        } else {
            // if the tree is empty, the new node must have no parent
            if self.nodes.is_empty() {
                if optional_parent_location.is_none() {
                    self.nodes.insert(node, GridTreeNode::new(None, 0));
                    self.leaves = vec![node];
                    Ok(())
                } else {
                    Err(error_handler::GridTreeError::ParentNotFound)
                }
            } else if let Some(parent_location) = optional_parent_location {
                let optional_parent_node = self.nodes.get_mut(&parent_location);
                match optional_parent_node {
                    None => Err(error_handler::GridTreeError::ParentNotFound),
                    Some(parent_node) => {
                        // if the parent was a leaf up to this point,
                        // remove it from the list of leaves
                        let parent_children_counter = &mut parent_node.children_counter;
                        if *parent_children_counter == 0 {
                            util_functions::remove_by_value(&parent_location, &mut self.leaves);
                        }
                        *parent_children_counter += 1;

                        let parent_depth = parent_node.depth;
                        self.leaves.push(node);
                        self.nodes.insert(
                            node,
                            GridTreeNode::new(Some(parent_location), parent_depth + 1),
                        );

                        Ok(())
                    }
                }
            } else {
                Err(error_handler::GridTreeError::NodeNotConnectedToTree)
            }
        }
    }

    /// if leaf_to_remove is None, picks one randomly
    /// returns the removed leaf, or None if there was a problem and the iter should stop
    fn remove_leaf(&mut self, leaf_to_remove: Option<GridLocation>) -> Option<GridLocation> {
        if self.leaves.is_empty() {
            return None;
        }
        let mut removed_leaf = match leaf_to_remove {
            Some(leaf_location) => {
                util_functions::remove_by_value(&leaf_location, &mut self.leaves);
                leaf_location
            }
            None => {
                let index_to_remove = util_functions::random_index(&self.leaves);
                self.leaves.remove(index_to_remove)
            }
        };
        let mut leaf_depth = self.nodes.get(&removed_leaf).unwrap().depth;
        // can't remove a leaf if its too close to the root to keep connectivity
        while leaf_depth < MINIMAL_DEPTH {
            if self.leaves.is_empty() {
                return None;
            } else {
                let index_to_remove = util_functions::random_index(&self.leaves);
                removed_leaf = self.leaves.remove(index_to_remove);
                leaf_depth = self.nodes.get(&removed_leaf).unwrap().depth;
            }
        }

        Some(removed_leaf)
    }

    /// intended to be used on leaves that were deemed valid
    /// to ensure their parents would become leaves too eventually
    /// returns true if the parent was found
    pub fn decrease_parent_child_count(&mut self, location: GridLocation) 
    -> Result<(), error_handler::GridTreeError> 
    {
        let optional_node_props = self.nodes.get(&location);
        match optional_node_props {
            None => Err(error_handler::GridTreeError::NodeNotFound),
            Some(node_props) => {
                let optional_parent_location = node_props.parent_location;
                if let Some(parent_location) = optional_parent_location {
                    let optional_parent_node = self.nodes.get_mut(&parent_location);
                    match optional_parent_node {
                        None => Err(error_handler::GridTreeError::ParentNotFound),
                        Some(parent_node) => {
                            parent_node.children_counter -= 1;
                            // if the parent is a leaf
                            if parent_node.children_counter == 0 {
                                // if the paren't depth is minimal,
                                // we don't want to eventually remove it
                                if parent_node.depth > MINIMAL_DEPTH {
                                    self.top_priority_leaf = Some(parent_location);
                                    self.leaves.push(parent_location);
                                }
                            }
                            Ok(())
                        }
                    }
                } else {
                    //could be that we, in some case, remove the root
                    Ok(())
                }
            }
        }
    }
}

impl Iterator for GridTree {
    type Item = GridLocation;

    fn next(&mut self) -> Option<Self::Item> {
        match self.top_priority_leaf {
            Some(top_priority_location) => {
                self.top_priority_leaf = None;
                Some(self.remove_leaf(Some(top_priority_location))?)
            }
            // if there's no top priority leaf, return a random leaf
            None => Some(self.remove_leaf(None)?),
        }
    }
}

// read-only functions
impl GridTree{
    pub fn get_grid_tree_node<'tree_lt>(&'tree_lt self, location: &GridLocation)
        -> Option<&'tree_lt GridTreeNode>
    {
        self.nodes.get(location)
    }
}

impl Default for GridTree {
    fn default() -> Self {
        Self::new()
    }
}

// //debug
// impl GridTree{
// 	fn print_leaves(&self, props_included: bool){
// 		let mut leaves_string = String::from("");
// 		for leaf in self.leaves.clone(){
// 			leaves_string += "\n";
// 			leaves_string += &leaf.to_string();
// 			if props_included{
// 				let leaf_prop = self.nodes.get(&leaf);
// 				if let Some(props) = leaf_prop{
// 					leaves_string += &(String::from(" ") + &props.to_string());
// 				}
// 			}
// 		}
// 		info!("{}", leaves_string);
// 	}

// 	fn print_nodes(&self){
// 		let mut nodes_string = String::from("");
// 		for node in self.nodes.clone(){
// 			nodes_string += "\n";
// 			nodes_string += &node.0.to_string();
// 			nodes_string += &(String::from(" ") + &node.1.to_string());
// 		}
// 		info!("{}", nodes_string);
// 	}
// }
