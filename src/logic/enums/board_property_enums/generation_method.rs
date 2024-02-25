use enum_iterator::{all, Sequence};
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum BoardGenerationMethod {
    #[default]
    Auto,
    Manual,
    Load,
}

impl BoardGenerationMethod {
    pub fn as_list() -> Vec<BoardGenerationMethod> {
        all::<BoardGenerationMethod>().collect::<Vec<BoardGenerationMethod>>()
    }
}

impl fmt::Display for BoardGenerationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}