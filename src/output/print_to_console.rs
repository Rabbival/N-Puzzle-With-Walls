use crate::prelude::*;

pub fn couldnt_generate_board(){
    panic!("couldn't generate board");
}

pub fn print_possible_solution<T: Iterator<Item = BasicDirection>>(reversed_directions_iter: T){
    info!("a possible solution would be:");
    for dir in reversed_directions_iter{
        info!("{:?}",dir);
    }
}