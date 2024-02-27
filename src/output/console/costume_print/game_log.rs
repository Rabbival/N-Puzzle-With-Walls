use crate::prelude::*;

pub enum GameLog<'a> {
    NewBoardGenerated,
    WallCountSet(u8),
    BoardSettingsChanged(&'a MenuButtonAction),
    TilesMoved(&'a Tile, &'a GridLocation),
    Victory,
}

pub fn game_log(log: GameLog) {
    match log {
        GameLog::NewBoardGenerated => {
            info!("a new board was generated!");
        }
        GameLog::WallCountSet(count) => {
            info!("wall count set: {:?}", count);
        }
        GameLog::BoardSettingsChanged(menu_button_action) => {
            info!("new setting set: {:?}", menu_button_action);
        }
        GameLog::TilesMoved(tile, location) => {
            info!("{:?} moved to {:?}", *tile, *location);
        }
        GameLog::Victory => {
            info!("puzzle solved!");
            info!("press R to restart");
        }
    }
}