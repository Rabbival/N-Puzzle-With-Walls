#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct IndexedValue<T: Copy + Eq + PartialEq + Default> {
    pub index: usize,
    pub value: T,
}

impl<T: Copy + Eq + PartialEq + Default> IndexedValue<T> {
    /// indexes with 0
    pub fn new(value: T) -> Self {
        Self { index: 0, value }
    }
}
