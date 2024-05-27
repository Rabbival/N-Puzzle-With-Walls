use std::fmt::{Display, Formatter};
use crate::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Hash, PartialOrd, Clone)]
pub enum PopUpMessageType {
    DeleteAllBoards,
    DeleteBoard(Option<(DomainBoardName, SavedLayoutIndexInDifficultyVec)>),
    ChooseNewbornDomainBoardName
}

impl Display for PopUpMessageType{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let pop_up_string = String::from("Are you sure you want to\n");
        let message =
            match self{
                Self::ChooseNewbornDomainBoardName => {
                    String::from("Choose a name\nto save the board under:")
                },
                Self::DeleteAllBoards => {
                    pop_up_string+
                    "delete all the boards?\n(Note that this will delete\ntheir save files as well)"
                },
                Self::DeleteBoard(optional_domain_board_to_delete) => {
                    match optional_domain_board_to_delete{
                        None => {
                            pop_up_string+
                            "do nothing?\n(please specify a board to delete)"
                        },
                        Some((board_name, _)) => {
                            pop_up_string+
                            &format!("delete {}?\n(Note that this will delete\nits save file as well)", board_name.0)
                        }
                    }
                },
            };
        fmt.write_str(&message)?;
        Ok(())
    }
}