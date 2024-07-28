use crate::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum GridError {
    InvalidIndex(GridLocation),
    InvalidPositionVector(Vec2),
}