use enum_iterator::{Sequence, all};
use crate::collect_all;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Sequence)]
pub enum SystemFileType{
    TextFile
}

impl SystemFileType{
    pub fn collect_all() -> Vec<Self> { collect_all!() }
    
    pub fn to_postfix(&self) -> String{
        match self{
            Self::TextFile => String::from(".txt"),
        }
    }
}