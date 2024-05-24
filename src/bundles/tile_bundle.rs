use bevy::render::view::RenderLayers;
use crate::prelude::*;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub on_screen_tags: MultipleOnScreenTags,
    pub loader_slot_ownership_tag: LoaderSlotOwnershipTag,
    pub render_layers: RenderLayers
}
