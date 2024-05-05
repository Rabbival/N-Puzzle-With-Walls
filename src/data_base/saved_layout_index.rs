use crate::prelude::*;

#[derive(Default, Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub struct SavedLayoutIndexInDifficultyVec {
    pub difficulty: BoardDifficulty,
    pub index_in_own_dif: usize
}

impl SavedLayoutIndexInDifficultyVec{
    pub fn from_screen_and_slot(
        board_difficulty: &BoardDifficulty,
        screen_and_slot: &LayoutLoaderScreenAndSlot
    ) -> Self
    {
        SavedLayoutIndexInDifficultyVec {
            difficulty: *board_difficulty,
            index_in_own_dif: screen_and_slot.screen*SAVED_LAYOUTS_PER_SCREEN
                + screen_and_slot.slot.to_layout_offset()
        }
    }
}