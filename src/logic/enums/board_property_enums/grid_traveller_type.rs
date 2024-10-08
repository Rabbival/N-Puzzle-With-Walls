use enum_iterator::{all, Sequence};
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::collect_all;

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum GridTravellerType {
    #[default]
    BFS,
    DFS,
}

impl GridTravellerType {
    pub fn collect_all() -> Vec<Self> { collect_all!() }

    pub fn to_button_option_text(&self) -> String{
        match *self {
            GridTravellerType::BFS => String::from("Scattered"),
            GridTravellerType::DFS => String::from("Chunky"),
        }
    }
}

impl fmt::Display for GridTravellerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}