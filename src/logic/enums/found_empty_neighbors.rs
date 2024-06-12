use crate::logic::data_structure::util_functions;
use crate::prelude::*;

#[derive(Debug)]
pub enum FoundEmptyNeighbors{
    OneEmptyNeighbor(BasicDirection, Tile),
    MoreThanOneEmptyNeighbor(HashMap<BasicDirection, Tile>),
    NoEmptyNeighbors
}

impl FoundEmptyNeighbors{
    pub fn from_empty_neighbors_map(empty_neighbors: HashMap<BasicDirection, Tile>) -> Self{
        match empty_neighbors.len(){
            2.. => Self::MoreThanOneEmptyNeighbor(empty_neighbors),
            1 => {
                let reference_pair = 
                    util_functions::get_single_key_value(&empty_neighbors).unwrap();
                Self::OneEmptyNeighbor(*reference_pair.0, *reference_pair.1)
            },
            0 => Self::NoEmptyNeighbors
        }
    }
}