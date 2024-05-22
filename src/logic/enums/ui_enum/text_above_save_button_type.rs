use std::fmt::{Display, Formatter};
use crate::{logic::DomainBoardName, prelude::SaveAttemptOutcome};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum TextAboveSaveButtonType {
    NoText,
    BoardAlreadyExistsInMemory(ExistingWallLayoutName),
    DataBaseAtCapacity,
    LayoutSavedSuccessfully(DomainBoardName)
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct ExistingWallLayoutName(pub String);

impl TextAboveSaveButtonType{
    pub fn from_save_attempt_outcome(save_attempt_outcome: SaveAttemptOutcome) -> Self {
        match save_attempt_outcome{
            SaveAttemptOutcome::LayoutSavedSuccessfully(domain_board_name) => {
                TextAboveSaveButtonType::LayoutSavedSuccessfully(domain_board_name)
            },
            SaveAttemptOutcome::BoardAlreadyExistsInMemory(existing_wall_layout_name) => {
                TextAboveSaveButtonType::BoardAlreadyExistsInMemory(existing_wall_layout_name)
            },
            SaveAttemptOutcome::DataBaseAtCapacity => {
                TextAboveSaveButtonType::DataBaseAtCapacity
            }
        }
    }
}

impl Display for TextAboveSaveButtonType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let message_string = match self{
            TextAboveSaveButtonType::NoText => {
                String::from("")
            },
            TextAboveSaveButtonType::BoardAlreadyExistsInMemory(ExistingWallLayoutName(existing_layout_name)) => {
                String::from("This walls layout is already saved under the name ") + existing_layout_name
            },
            TextAboveSaveButtonType::DataBaseAtCapacity => {
                String::from("Layouts memory at capacity, Delete some to save new ones.")
            },
            TextAboveSaveButtonType::LayoutSavedSuccessfully(domain_board_name) => {
                String::from("Walls layout saved successfully as ")+&domain_board_name.0
            },
        };
        fmt.write_str(&message_string)?;
        Ok(())
    }
}