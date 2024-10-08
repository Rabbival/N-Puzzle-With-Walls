use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum MenuButtonAction {
    ChangeSize(BoardSize),
    ChangeWallTilesCount(WallTilesChange),
    ChangeEmptyTilesCount(u8),
    ChangeGenerationMethod(BoardGenerationMethod),
    ChangeSpanningTreeGeneration(GridTravellerType),
    ChangeBoardDifficulty(BoardDifficulty),
    MainButtonPressed,
}