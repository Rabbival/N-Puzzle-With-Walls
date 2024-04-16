use enum_iterator::Sequence;
use crate::prelude::SaveAttemptOutcome;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAboveSaveButtonType {
    NoText,
    WallLayoutAlreadyExistsInMemory,
    WallsLayoutsAtCapacity,
    LayoutSavedSuccessfully
}

impl TextAboveSaveButtonType{
    pub fn from_save_attempt_outcome(save_attempt_outcome: SaveAttemptOutcome) -> Self {
        match save_attempt_outcome{
            SaveAttemptOutcome::LayoutSavedSuccessfully => {
                TextAboveSaveButtonType::LayoutSavedSuccessfully
            },
            SaveAttemptOutcome::WallLayoutAlreadyExistsInMemory => {
                TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory
            },
            SaveAttemptOutcome::WallsLayoutsAtCapacity => {
                TextAboveSaveButtonType::WallsLayoutsAtCapacity
            }
        }
    }
    
    pub fn to_string(&self) -> String{
        match self{
            TextAboveSaveButtonType::NoText => {
                String::from("")
            },
            TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory => {
                String::from("This walls layout is already saved")
            },
            TextAboveSaveButtonType::WallsLayoutsAtCapacity => {
                String::from("Layouts memory at capacity, Delete some to save new ones.")
            },
            TextAboveSaveButtonType::LayoutSavedSuccessfully => {
                String::from("Walls layout saved successfully!")
            },
        }
    }
}