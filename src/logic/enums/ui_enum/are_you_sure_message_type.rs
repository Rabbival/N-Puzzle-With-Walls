use std::fmt::{Display, Formatter};
use enum_iterator::Sequence;

#[derive(Debug, Sequence, PartialEq, Eq, Hash, PartialOrd, Clone, Copy)]
pub enum AreYouSureMessageType {
    ClearDataBase,
    DeleteBoard
}

impl Display for AreYouSureMessageType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(match self{
            AreYouSureMessageType::ClearDataBase => {
                ""
            },
            AreYouSureMessageType::DeleteBoard => {
                ""
            },
        })?;
        Ok(())
    }
}