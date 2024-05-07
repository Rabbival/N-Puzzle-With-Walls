use crate::prelude::{BoardDifficulty, LayoutLoaderScreenAndSlot};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct ScreenSlotAndDifficulty{
    pub screen_and_slot: LayoutLoaderScreenAndSlot,
    pub difficulty: BoardDifficulty
}