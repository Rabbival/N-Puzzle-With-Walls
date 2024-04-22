use std::fmt::{Display, Formatter};
use enum_iterator::Sequence;
use crate::prelude::{SaveAttemptOutcome};

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
}

impl Display for TextAboveSaveButtonType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self{
                TextAboveSaveButtonType::NoText => {
                    ""
                },
                TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory => {
                    "This walls layout is already saved"
                },
                TextAboveSaveButtonType::WallsLayoutsAtCapacity => {
                    "Layouts memory at capacity, Delete some to save new ones."
                },
                TextAboveSaveButtonType::LayoutSavedSuccessfully => {
                    "Walls layout saved successfully!"
                },
            })?;
        Ok(())
    }
}