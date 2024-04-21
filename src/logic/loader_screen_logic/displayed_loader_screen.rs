use crate::prelude::*;

#[derive(Resource, Default)]
pub struct DisplayedLoaderScreenNumber(pub usize);

pub struct DisplayedLoaderScreenPlugin;

impl Plugin for DisplayedLoaderScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DisplayedLoaderScreenNumber>();
    }
}

pub fn get_layout_index_by_screen_and_slot(screen: usize, slot: LoaderScreenSlot) -> SavedLayoutIndex {
    SavedLayoutIndex(screen*SAVED_LAYOUTS_PER_SCREEN + slot.to_layout_offset())
}

pub fn try_get_layout_screen_and_slot_by_index(index: SavedLayoutIndex) -> Option<LayoutLoaderScreenAndSlot> {
    Some(LayoutLoaderScreenAndSlot{
        screen: index.0 / SAVED_LAYOUTS_PER_SCREEN,
        slot: LoaderScreenSlot::try_from_layout_offset(index.0 % SAVED_LAYOUTS_PER_SCREEN)?
    })
}