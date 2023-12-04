#[derive(Debug)]
pub enum InputHandlerErrors{
    IndexOutOfGridBounds,
    GridLocationOccupied,
}

#[derive(Debug)]
pub enum DirectionRelatedErrors{
    DirectionCouldntBeFlipped,
    DirectionNotFoundInMap
}