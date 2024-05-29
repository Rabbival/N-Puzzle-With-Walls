use std::fmt::{Formatter, Display};
use crate::prelude::*;
use crate::prelude::game_session_log::append_to_game_session_log_file;

pub enum GameLog<'a> {
    NewBoardGenerated,
    WallCountSet(u8),
    BoardSettingsChanged(&'a MenuButtonAction),
    TilesMoved(&'a Tile, &'a GridLocation),
    Victory,
}

pub fn game_log(game_log: GameLog) {
    append_to_game_session_log_file(game_log.to_string());
    info!("{}", game_log.to_string());
}


impl Display for GameLog<'_>{
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let message_string =  match self {
            GameLog::NewBoardGenerated => {
                String::from("\na new board was generated!\n")
            }
            GameLog::WallCountSet(count) => {
                format!("wall count set: {:?}", count)
            }
            GameLog::BoardSettingsChanged(menu_button_action) => {
                format!("new setting set: {:?}", menu_button_action)
            }
            GameLog::TilesMoved(tile, location) => {
                format!("{:?} moved to {:?}", *tile, *location)
            }
            GameLog::Victory => {
                String::from("puzzle solved!\npress R to restart")
            }
        };
        fmt.write_str(&message_string)?;
        Ok(())
    }
}