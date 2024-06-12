use std::fmt::{Display, Formatter};
use crate::prelude::*;


#[derive(Component, Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum GameScreenTextType {
    NoText,
    BoardAlreadyExistsInMemory(ExistingWallLayoutName),
    DataBaseAtCapacity,
    LayoutSavedSuccessfully(DomainBoardName),
    CouldntGenerateBoard,
    EmptyTileChoicePending
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct ExistingWallLayoutName(pub String);

impl GameScreenTextType{
    pub fn from_save_attempt_outcome(save_attempt_outcome: SaveAttemptOutcome) -> Self {
        match save_attempt_outcome{
            SaveAttemptOutcome::LayoutSavedSuccessfully(domain_board_name) => {
                GameScreenTextType::LayoutSavedSuccessfully(domain_board_name)
            },
            SaveAttemptOutcome::BoardAlreadyExistsInMemory(existing_wall_layout_name) => {
                GameScreenTextType::BoardAlreadyExistsInMemory(existing_wall_layout_name)
            },
            SaveAttemptOutcome::DataBaseAtCapacity => {
                GameScreenTextType::DataBaseAtCapacity
            },
        }
    }
}

impl Display for GameScreenTextType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let message_string = match self{
            GameScreenTextType::NoText => {
                String::from("")
            },
            GameScreenTextType::BoardAlreadyExistsInMemory(ExistingWallLayoutName(existing_layout_name)) => {
                String::from("Wall layout already saved as ") + existing_layout_name
            },
            GameScreenTextType::DataBaseAtCapacity => {
                String::from("Layouts memory at capacity")
            },
            GameScreenTextType::LayoutSavedSuccessfully(domain_board_name) => {
                String::from("Walls layout saved successfully as ")+&domain_board_name.0
            },
            GameScreenTextType::CouldntGenerateBoard => {
                String::from("Failed to generate a new board")
            },
            GameScreenTextType::EmptyTileChoicePending => {
                String::from("Choose an empty tile to switch with")
            }
        };
        fmt.write_str(&message_string)?;
        Ok(())
    }
}