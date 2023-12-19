#[derive(Clone, Copy)]
pub struct IndexedValue<T: Copy>{
    pub index: usize,
    pub value: T
}