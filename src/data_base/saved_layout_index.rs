use crate::prelude::{LayoutLoaderScreenAndSlot, LoaderScreenSlot, SAVED_LAYOUTS_PER_SCREEN};

#[derive(Default, Debug)]
pub struct SavedLayoutIndex(pub usize);

impl SavedLayoutIndex{
    pub fn from_screen_and_slot(screen: usize, slot: LoaderScreenSlot) -> Self {
        SavedLayoutIndex(screen*SAVED_LAYOUTS_PER_SCREEN + slot.to_layout_offset())
    }

    pub fn try_to_layout_screen_and_slot(&self) -> Option<LayoutLoaderScreenAndSlot> {
        Some(LayoutLoaderScreenAndSlot{
            screen: self.0 / SAVED_LAYOUTS_PER_SCREEN,
            slot: LoaderScreenSlot::try_from_layout_offset(self.0 % SAVED_LAYOUTS_PER_SCREEN)?
        })
    }
}