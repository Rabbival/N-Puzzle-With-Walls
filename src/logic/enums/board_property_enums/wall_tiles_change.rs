use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum WallTilesChange {
    Increase,
    Decrease,
    Apply,
}
