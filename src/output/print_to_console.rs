use std::fmt::Debug;

use crate::prelude::*;

pub enum GameLog<'a>{
    NewBoardGenerated,
    BoardSettingsChanged(&'a MenuButtonAction),
    TilesMoved(&'a Tile, &'a GridLocation),
    Victory
}

pub fn game_log(log: GameLog){
    match log{
        GameLog::NewBoardGenerated=>{
            info!("a new board was generated!");
        },
        GameLog::BoardSettingsChanged(menu_button_action)=>{
            info!("new setting set: {:?}", menu_button_action);
        },
        GameLog::TilesMoved(indexed_tile_type, location)=>{
            info!("{:?} tile moved to {:?}", *indexed_tile_type, *location);
        },
        GameLog::Victory=>{
            info!("puzzle solved!");
            info!("press R to restart");
        }
    }
}

pub enum BevyPrintType{
    Info, 
    Warn,
    Error
}

pub fn print_debug_deriver<T: Debug>(to_print: T, print_type: BevyPrintType){
    match print_type{
        BevyPrintType::Info=> {info!("{:?}", to_print)},
        BevyPrintType::Warn=> {warn!("{:?}", to_print)},
        BevyPrintType::Error=> {error!("{:?}", to_print)}
    }
}

pub fn print_menu_error(menu_error: MenuError){
    match menu_error{
        MenuError::CantGoBeyondTileCountBounds(attempted_change)=> {
            warn!("attempted change {:?} can't execute due to predefined bounds", attempted_change);
        }
    }
}

pub fn couldnt_generate_board(){
    panic!("couldn't generate board");
}

pub fn print_tile_move_error(move_error: TileMoveError){
    match move_error{
        TileMoveError::BoardFrozenToPlayer(message)=>{
            warn!(message);
        },
        TileMoveError::NoEmptyNeighbor(message)=>{
            warn!(message);
        },
        TileMoveError::PressedEmptySlot(message)=>{
            warn!(message);
        },
        TileMoveError::NoOccupiedTileInThatDirection(direction)=>{
            warn!("no occupied tile in direction: {:?}", direction);
        },
        TileMoveError::IndexOutOfGridBounds(message)=>{
            warn!(message);
        },
        _=>{error!("{:?}", move_error)}
    }
}

pub fn print_entity_related_error(entity_error: EntityRelatedCustomError){
    error!("{:?}", entity_error);
}

pub fn print_possible_solution<T: Iterator<Item = BasicDirection>>(reversed_directions_iter: T){
    info!("a possible solution would be:");
    for dir in reversed_directions_iter{
        info!("{:?}",dir);
    }
}