use enum_iterator::{all, Sequence};
use std::fmt;
use serde::{Deserialize, Serialize};
use crate::collect_all;
use crate::prelude::AppState;

#[derive(Sequence, Debug, Clone, Copy, Default, PartialEq, Eq, Deserialize, Serialize)]
pub enum BoardGenerationMethod {
    #[default]
    Auto,
    Manual,
    Load,
}

impl BoardGenerationMethod {
    pub fn collect_all() -> Vec<Self> {collect_all!()}
    
    pub fn to_generation_button_text(&self) -> String {
        match self{
            BoardGenerationMethod::Auto => String::from("Generate"),
            BoardGenerationMethod::Manual => String::from("Build"),
            BoardGenerationMethod::Load => String::from("Load"),
        }
    }
    
    pub fn to_app_state(&self) -> AppState {
        match self{
            BoardGenerationMethod::Auto => AppState::Game,
            BoardGenerationMethod::Manual => AppState::Manual,
            BoardGenerationMethod::Load => AppState::Loader,
        }
    }
}

impl fmt::Display for BoardGenerationMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}