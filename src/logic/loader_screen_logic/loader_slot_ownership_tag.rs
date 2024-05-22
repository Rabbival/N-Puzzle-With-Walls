use crate::prelude::*;

#[derive(Component, Debug, Default, Copy, Clone)]
pub struct LoaderSlotOwnershipTag(pub Option<LoaderScreenSlot>);

impl LoaderSlotOwnershipTag{
    pub fn to_render_layer(&self) -> u8 {
        match self.0{
            Some(loader_slot) => {
                (loader_slot as u8) + 1
            },
            None => 0
        }
    }
}