use std::fmt::{Display, Formatter};
use crate::prelude::{SaveAttemptOutcome};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum TextAboveSaveButtonType {
    NoText,
    WallLayoutAlreadyExistsInMemory(ExistingWallLayoutName),
    WallsLayoutsAtCapacity,
    LayoutSavedSuccessfully
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct ExistingWallLayoutName(pub String);

impl TextAboveSaveButtonType{
    pub fn from_save_attempt_outcome(save_attempt_outcome: SaveAttemptOutcome) -> Self {
        match save_attempt_outcome{
            SaveAttemptOutcome::LayoutSavedSuccessfully => {
                TextAboveSaveButtonType::LayoutSavedSuccessfully
            },
            SaveAttemptOutcome::WallLayoutAlreadyExistsInMemory(existing_wall_layout_name) => {
                TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory(existing_wall_layout_name)
            },
            SaveAttemptOutcome::WallsLayoutsAtCapacity => {
                TextAboveSaveButtonType::WallsLayoutsAtCapacity
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
            TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory(ExistingWallLayoutName(existing_layout_name)) => {
                String::from("This walls layout is already saved under the name ") + existing_layout_name
            },
            TextAboveSaveButtonType::WallsLayoutsAtCapacity => {
                String::from("Layouts memory at capacity, Delete some to save new ones.")
            },
            TextAboveSaveButtonType::LayoutSavedSuccessfully => {
                String::from("Walls layout saved successfully!")
            },
        };
        fmt.write_str(&message_string)?;
        Ok(())
    }
}