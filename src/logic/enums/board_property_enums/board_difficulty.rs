use enum_iterator::{all, Sequence};
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::collect_all;

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum BoardDifficulty {
    Easy,
    #[default]
    Medium,
    Hard,
}

impl BoardDifficulty {
    pub fn collect_all() -> Vec<Self> { collect_all!() }

    pub fn to_random_turns_multiplier(&self) -> f32 {
        match *self {
            BoardDifficulty::Easy => 0.5,
            BoardDifficulty::Medium => 1.0,
            BoardDifficulty::Hard => 1.5,
        }
    }
}

impl fmt::Display for  BoardDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}