use enum_iterator::all;
use crate::prelude::*;

pub struct SavedLayoutsScreen(pub HashMap<LoaderScreenSlot, Option<DomainBoardNameWithoutPostfix>>);

impl Default for SavedLayoutsScreen{
    fn default() -> Self {
        let mut hashmap = HashMap::new();
        for loader_screen_slot in all::<LoaderScreenSlot>(){
            hashmap.insert(loader_screen_slot, None);
        }
        Self(hashmap)
    }
}
