#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct IndexedValue<T: Copy + Eq + PartialEq + Default>{
    pub index: usize,
    pub value: T
}