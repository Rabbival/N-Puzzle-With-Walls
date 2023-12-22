#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BoardGenerationMethod{
    #[default]
    Automatic,
    Manual,
    FromDataBase
}