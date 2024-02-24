use enum_iterator::{all, Sequence};
use std::fmt;
use json::JsonValue;

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
}

impl fmt::Display for GridTravellerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Into<JsonValue> for GridTravellerType {
    fn into(self) -> JsonValue {
        match self {
            GridTravellerType::BFS => "BFS".into(),
            GridTravellerType::DFS => "DFS".into(),
        }
    }
}