use crate::prelude::*;

#[derive(Bundle)]
pub struct SavedLayoutBundle {
    pub domain_board: DomainBoard,
    pub tile_board: TileBoard,
}