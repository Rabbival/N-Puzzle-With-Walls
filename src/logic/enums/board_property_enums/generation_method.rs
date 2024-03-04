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
    
    pub fn to_generation_button_text(&self) -> String {
        match self{
            BoardGenerationMethod::Auto => String::from("Generate"),
            BoardGenerationMethod::Manual => String::from("Build"),
            BoardGenerationMethod::Load => String::from("Load"),
        }
    }
}

impl fmt::Display for BoardGenerationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}