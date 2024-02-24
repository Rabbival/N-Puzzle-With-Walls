use enum_iterator::{all, Sequence};
use std::fmt;

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum GridTravellerType {
    #[default]
    BFS,
    DFS,
}

impl GridTravellerType {
    pub fn as_list() -> Vec<GridTravellerType> {
        all::<GridTravellerType>().collect::<Vec<GridTravellerType>>()
    }

    fn to_json(&self) -> serde_json::Value {
        match self {
            GridTravellerType::BFS => "BFS".into(),
            GridTravellerType::DFS => "DFS".into(),
        }
    }
}

impl fmt::Display for GridTravellerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}