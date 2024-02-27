use crate::prelude::*;

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