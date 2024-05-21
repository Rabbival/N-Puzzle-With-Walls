use crate::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub on_screen_tag: CustomOnScreenTag,
    pub loader_slot_ownership_tag: LoaderSlotOwnershipTag
}
