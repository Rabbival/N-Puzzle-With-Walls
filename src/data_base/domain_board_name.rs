use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub struct DomainBoardName(pub String);

impl fmt::Display for DomainBoardName{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)?;
        Ok(())
    }
}