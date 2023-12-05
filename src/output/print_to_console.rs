use crate::prelude::*;

pub enum GameLog{
    TileClicked(TileType),
    TilesMoved(GridLocation),
    Victory
}

pub fn couldnt_generate_board(){
    panic!("couldn't generate board");
}

pub fn print_input_error(input_error: TileMoveError){
    match input_error{
        TileMoveError::BoardFrozenToPlayer(message)=>{
            warn!(message);
        },
        TileMoveError::NoEmptyNeighbor(message)=>{
            warn!(message);
        },
        TileMoveError::PressedEmptySlot(message)=>{
            warn!(message);
        },
        TileMoveError::IndexOutOfGridBounds(message)=>{
            error!(message);
        },
        _=>{error!("{:?}", input_error)}
    }
}

pub fn print_possible_solution<T: Iterator<Item = BasicDirection>>(reversed_directions_iter: T){
    info!("a possible solution would be:");
    for dir in reversed_directions_iter{
        info!("{:?}",dir);
    }
}

pub fn game_log(log: GameLog){
    match log{
        GameLog::TileClicked(tile_type)=>{
            info!("clicked tile: {:?}", tile_type);
        },
        GameLog::TilesMoved(location)=>{
            info!("tile moved to: {:?}", location);
        },
        GameLog::Victory=>{
            info!("puzzle solved!");
            info!("press R to restart");
        }
    }
}