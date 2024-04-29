use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum BoardBuildingRequest{
    ShuffleExistingBoard,
    CreateANewBoardFromNothing,
    CreateANewBoardFromTileBoardWithWalls(TileBoard)
}