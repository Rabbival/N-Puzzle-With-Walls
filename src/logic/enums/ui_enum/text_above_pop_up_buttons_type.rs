use std::fmt::{Display, Formatter};
use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAbovePopUpButtonsType {
    NoText,
    BoardNameAlreadyExists,
    CantHaveALongerName,
    MustGiveAName
}

impl Display for TextAbovePopUpButtonsType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(
            match self{
                Self::NoText => {
                    ""
                },
                Self::BoardNameAlreadyExists => {
                    "Note: This board name already exists,\nSaving will overwrite the existing board"
                },
                Self::CantHaveALongerName => {
                    " Can't have a longer name "
                },
                Self::MustGiveAName => {
                    " Please provide a name for the board "
                }
            })?;
        Ok(())
    }
}