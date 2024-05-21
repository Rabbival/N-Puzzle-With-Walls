use crate::prelude::*;

const PRINT_POSSIBLE_SOLUTION_WHEN_ASKED_TO: bool = false;

pub fn print_possible_solution<T: Iterator<Item = BasicDirection>>(
    empty_tile_index: usize,
    reversed_directions_iter: T,
) {
    if PRINT_POSSIBLE_SOLUTION_WHEN_ASKED_TO {
        info!(
            "for empty tile no.{}, a possible solution would be:",
            empty_tile_index
        );
        for dir in reversed_directions_iter {
            info!("{:?}", dir);
        }
    }
}