use std::fmt::{Display, Formatter};
use crate::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum AreYouSureMessageType {
    DeleteAllBoards,
    DeleteBoard(Option<(DomainBoardName, SavedLayoutIndexInDifficultyVec)>)
}

impl Display for AreYouSureMessageType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let message = String::from("Are you sure you want to\n") +
            &match self{
                AreYouSureMessageType::DeleteAllBoards => {
                    String::from("delete all the boards?\n(Note that this will delete\ntheir save files as well)")
                },
                AreYouSureMessageType::DeleteBoard(optional_domain_board_to_delete) => {
                    match optional_domain_board_to_delete{
                        None => {
                            String::from("do nothing?\n(please specify a board to delete)")
                        },
                        Some((board_name, _)) => {
                            format!("delete {}?\n(Note that this will delete\nits save file as well)", board_name.0)
                        }
                    }
                },
            };
        fmt.write_str(&message)?;
        Ok(())
    }
}