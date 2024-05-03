use std::fmt;
use std::fmt::Formatter;
use bevy::prelude::Component;

#[derive(Component, Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Default)]
pub struct DomainBoardName(pub String);

impl DomainBoardName{
    pub fn to_string_for_button(&self) -> String {
        let mut string_for_button = String::from("'");
        string_for_button += &self.0;
        string_for_button += "'";
   
        string_for_button
    }
}

impl fmt::Display for DomainBoardName{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)?;
        Ok(())
    }
}