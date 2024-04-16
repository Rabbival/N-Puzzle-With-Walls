#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SaveAttemptOutcome {
    WallLayoutAlreadyExistsInMemory,
    WallsLayoutsAtCapacity,
    LayoutSavedSuccessfully
}