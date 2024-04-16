use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAboveStartButtonType {
    NoText,
    UnappliedChanges,
    CouldntGenerateBoard,
    TriedLoadingAnInvalidBoard
}

impl TextAboveStartButtonType{
    pub fn to_string(&self) -> String{
        match self{
            TextAboveStartButtonType::NoText => {
                String::from("")
            },
            TextAboveStartButtonType::UnappliedChanges => {
                String::from(" Note: you have unapplied changes ")
            },
            TextAboveStartButtonType::CouldntGenerateBoard => {
                String::from(" No board could be generated ")
            },
            TextAboveStartButtonType::TriedLoadingAnInvalidBoard => {
                String::from(" Tried loading an invalid board ")
            },
        }
    }
}