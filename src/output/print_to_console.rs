use std::fmt::Display;

use crate::prelude::*;

pub fn print_board_generation_error(error: BoardGenerationError) {
    error!("board generation failed! error: {:?}", error);
}

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

pub enum BevyPrintType {
    Info,
    Warn,
    Error,
}

pub fn print_display_deriver_vec<T: Display>(to_print: &Vec<T>, print_type: BevyPrintType) {
    let mut to_print_str = String::from("[");
    for item in to_print {
        to_print_str += &(String::from(" ") + &item.to_string());
    }
    to_print_str += " ]";
    match print_type {
        BevyPrintType::Info => {
            info!("{}", to_print_str)
        }
        BevyPrintType::Warn => {
            warn!("{}", to_print_str)
        }
        BevyPrintType::Error => {
            error!("{}", to_print_str)
        }
    }
}

pub fn print_menu_error(menu_error: MenuError) {
    match menu_error {
        MenuError::CantGoBeyondTileCountBounds(attempted_change) => {
            warn!(
                "attempted change {:?} can't execute due to predefined bounds",
                attempted_change
            );
        }
    }
}

pub fn print_tile_move_error(move_error: TileMoveError) {
    match move_error {
        TileMoveError::BoardFrozenToPlayer => {
            warn!("board locked");
        }
        TileMoveError::NoEmptyNeighbor => {
            warn!("no empty neighbor");
        }
        TileMoveError::PressedEmptySlot => {
            warn!("pressed an empty slot");
        }
        TileMoveError::NoOccupiedTileInThatDirection(direction) => {
            warn!("no occupied tile in direction: {:?}", direction);
        }
        TileMoveError::TriedToSwitchEmptyWithEmpty => {
            info!("tried to switch empty with empty, hence no swap was performed");
        }
        _ => {
            error!("{:?}", move_error)
        }
    }
}

pub fn print_entity_related_error(entity_error: EntityRelatedCustomError) {
    error!("{:?}", entity_error);
}

pub fn print_possible_solution<T: Iterator<Item = BasicDirection>>(
    empty_tile_index: usize,
    reversed_directions_iter: T,
) {
    info!(
        "for empty tile no.{}, a possible solution would be:",
        empty_tile_index
    );
    for dir in reversed_directions_iter {
        info!("{:?}", dir);
    }
}

#[derive(Debug)]
pub enum SystemLog{
    FolderCreated,
    FolderExists,
    RequestedFileDoesntExist,
}

pub fn print_system_log(system_log: SystemLog){
    info!("{:?}", system_log);
}