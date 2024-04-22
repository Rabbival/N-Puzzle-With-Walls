use std::fmt::{Display, Formatter};
use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum TextAboveStartButtonType {
    NoText,
    UnappliedChanges,
    CouldntGenerateBoard,
    TriedLoadingAnInvalidBoard
}

impl Display for TextAboveStartButtonType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(
            match self{
                TextAboveStartButtonType::NoText => {
                    ""
                },
                TextAboveStartButtonType::UnappliedChanges => {
                    " Note: you have unapplied changes "
                },
                TextAboveStartButtonType::CouldntGenerateBoard => {
                    " No board could be generated "
                },
                TextAboveStartButtonType::TriedLoadingAnInvalidBoard => {
                    " Tried loading an invalid board "
                },
            })?;
        Ok(())
    }
}