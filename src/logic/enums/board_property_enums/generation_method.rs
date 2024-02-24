use enum_iterator::{all, Sequence};
use std::fmt;
use json::JsonValue;

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq)]
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

impl Into<JsonValue> for BoardGenerationMethod {
    fn into(self) -> JsonValue {
        match self {
            BoardGenerationMethod::Auto => "Auto".into(),
            BoardGenerationMethod::Manual => "Manual".into(),
            BoardGenerationMethod::Load => "Load".into(),
        }
    }
}