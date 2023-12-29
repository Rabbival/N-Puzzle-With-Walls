#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IndexedValue<T: Copy + Eq + PartialEq>{
    pub index: usize,
    pub value: T
}

// impl<T: Copy> IndexedValue<T>{
//     pub fn to_tuple(&self) -> (T, T) {
//         (self.index, self.value)
//     }
// }