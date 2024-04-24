use std::fmt::{Display, Formatter};
use crate::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum AreYouSureMessageType {
    DeleteAllBoards,
    DeleteBoard(DomainBoardName, SavedLayoutIndex)
}

impl Display for AreYouSureMessageType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let message = String::from("Are you sure you want to\n") +
            &match self{
                AreYouSureMessageType::DeleteAllBoards => {
                    String::from("delete all the boards?\n(Note that this will delete\ntheir save files as well)")
                },
                AreYouSureMessageType::DeleteBoard(domain_board_to_delete, _) => {
                    format!("delete {}?\n(Note that this will delete\nits save file as well)", domain_board_to_delete.0)
                },
            };
        fmt.write_str(&message)?;
        Ok(())
    }
}