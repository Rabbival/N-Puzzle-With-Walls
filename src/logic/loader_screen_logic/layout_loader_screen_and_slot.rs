use crate::prelude::{LoaderScreenSlot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutLoaderScreenAndSlot{
    pub screen: usize,
    pub slot: LoaderScreenSlot
}
