use crate::prelude::{LoaderScreenSlot, SAVED_LAYOUTS_PER_SCREEN};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutLoaderScreenAndSlot{
    pub screen: usize,
    pub slot: LoaderScreenSlot
}

impl LayoutLoaderScreenAndSlot{
    pub fn increment(&mut self){
        let incremented_slot_num_value = self.slot.to_layout_offset() + 1;
        let new_slot = 
            LoaderScreenSlot::try_from_layout_offset(
                incremented_slot_num_value % SAVED_LAYOUTS_PER_SCREEN
            ).unwrap();
        let new_screen = 
            if incremented_slot_num_value > SAVED_LAYOUTS_PER_SCREEN { self.screen + 1 } else { self.screen };
        self.slot = new_slot; 
        self.screen = new_screen;
    }

    pub fn decrement_if_possible(&mut self){
        let decremented_slot_num_value = self.slot.to_layout_offset() as isize - 1;
        let new_slot =
            LoaderScreenSlot::try_from_layout_offset(
                (decremented_slot_num_value + SAVED_LAYOUTS_PER_SCREEN as isize) 
                            as usize % SAVED_LAYOUTS_PER_SCREEN
            ).unwrap();
        let new_screen =
            if self.slot.to_layout_offset() == 0 && self.screen > 0 { self.screen - 1 } else { self.screen };
        self.slot = new_slot;
        self.screen = new_screen;
    }
}