use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAboveSaveButtonType {
    NoText,
    WallLayoutAlreadyExistsInMemory,
    WallsLayoutsAtCapacity,
}

impl TextAboveSaveButtonType{
    pub fn to_string(&self) -> String{
        match self{
            TextAboveSaveButtonType::NoText => {
                String::from("")
            },
            TextAboveSaveButtonType::WallLayoutAlreadyExistsInMemory => {
                String::from("This walls layout is already saved")
            },
            TextAboveSaveButtonType::WallsLayoutsAtCapacity => {
                String::from("Layouts memory at capacity, Delete some to save new ones.")
            }
        }
    }
}