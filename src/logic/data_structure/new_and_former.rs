#[derive(Clone, Copy, Default)]
pub struct NewAndFormer<T: Copy + Default>{
    pub new: T,
    pub former: T
}