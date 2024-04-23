use std::fmt::{Display, Formatter};
use enum_iterator::Sequence;
use crate::prelude::{MenuError, WallTilesChange};

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAboveStartButtonType {
    NoText,
    UnappliedChanges,
    CouldntGenerateBoard,
    TriedLoadingAnInvalidBoard,
    MenuError(MenuError)
}

impl Display for TextAboveStartButtonType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(
            match self{
                TextAboveStartButtonType::NoText => {
                    ""
                },
                TextAboveStartButtonType::UnappliedChanges => {
                    " Note: you have unapplied changes "
                },
                TextAboveStartButtonType::CouldntGenerateBoard => {
                    " No board could be generated "
                },
                TextAboveStartButtonType::TriedLoadingAnInvalidBoard => {
                    " Tried loading an invalid board "
                },
                TextAboveStartButtonType::MenuError(menu_error) => {
                    match menu_error{
                        MenuError::CantGoBeyondTileCountBounds(wall_tiles_change) => {
                            match wall_tiles_change {
                                WallTilesChange::Increase => {
                                    " can't add more walls, try increasing the board size"
                                },
                                WallTilesChange::Decrease => {
                                    " can't have a negative number of walls on board"
                                },
                                WallTilesChange::Apply => { "" }
                            }
                        }
                    }
                }
            })?;
        Ok(())
    }
}