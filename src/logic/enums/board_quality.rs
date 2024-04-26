use enum_iterator::Sequence;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Sequence)]
pub enum BoardQuality {
    Invalid,
    NotFullyConnected,
    SomeTilesHaveLessThanTwoNeighbors,
    NotAllTilesAreAPartOfACycle,
    BestQuality
}