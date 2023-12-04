use crate::prelude::*;

pub enum GameLog{
    TileClicked(GridLocation),
}

pub fn couldnt_generate_board(){
    panic!("couldn't generate board");
}

pub fn print_possible_solution<T: Iterator<Item = BasicDirection>>(reversed_directions_iter: T){
    info!("a possible solution would be:");
    for dir in reversed_directions_iter{
        info!("{:?}",dir);
    }
}

pub fn game_log(log: GameLog){
    match log{
        GameLog::TileClicked(location)=>{
            info!("clicked location: {:?}", location)
        },
    }
}