use std::fmt::{Display, Formatter};
use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAbovePopUpButtonsType {
    NoText,
    BoardNameAlreadyExists
}

impl Display for TextAbovePopUpButtonsType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(
            match self{
                Self::NoText => {
                    ""
                },
                Self::BoardNameAlreadyExists => {
                    " This board name already exists "
                }
            })?;
        Ok(())
    }
}