use crate::{logic::DomainBoardName, prelude::ExistingWallLayoutName};

#[derive(Debug, Clone, PartialEq)]
pub enum SaveAttemptOutcome {
    BoardAlreadyExistsInMemory(ExistingWallLayoutName),
    DataBaseAtCapacity,
    LayoutSavedSuccessfully(DomainBoardName)
}