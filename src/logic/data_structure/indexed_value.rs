#[derive(Clone, Copy)]
pub struct IndexedValue<T: Copy>{
    pub index: usize,
    pub value: T
}

// impl<T: Copy> IndexedValue<T>{
//     pub fn to_tuple(&self) -> (T, T) {
//         (self.index, self.value)
//     }
// }