use enum_iterator::Sequence;
use crate::prelude::*;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum MenuError {
    CantGoBeyondTileCountBounds(WallTilesChange),
}

pub fn print_menu_error(menu_error: MenuError) {
    match menu_error {
        MenuError::CantGoBeyondTileCountBounds(attempted_change) => {
            warn!(
                "attempted change {:?} can't execute due to predefined bounds",
                attempted_change
            );
        }
    }
}