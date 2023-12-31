use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub enum MenuButtonAction{
    ChangeSize(BoardSize),
    ChangeWallTilesCount(WallTilesChange),
    ChangeEmptyTilesCount(u8),
    ChangeGenerationMethod(BoardGenerationMethod),
    GenerateBoard,
    EndGame
}