use crate::prelude::ExistingWallLayoutName;

#[derive(Debug, Clone, PartialEq)]
pub enum SaveAttemptOutcome {
    WallLayoutAlreadyExistsInMemory(ExistingWallLayoutName),
    WallsLayoutsAtCapacity,
    LayoutSavedSuccessfully
}