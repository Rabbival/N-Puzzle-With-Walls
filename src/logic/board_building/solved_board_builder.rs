use crate::{logic::data_structure::util_functions, output::error_handler, prelude::*};

/// mustn't be more than 2 as there will always be a corner
/// shouldn't be less than 1 or we might get useless spaces
const MIN_NEIGHBORS: u8 = 2;

pub fn generate_solved_board(
    applied_props: &BoardProperties,
) -> Result<TileTypeBoard, BoardGenerationError> {
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;

    if applied_props.wall_count > 0 {
        let wall_locations = determine_wall_locations(applied_props)?;
        spawn_walls_in_locations(wall_locations, &mut solved_board);
    }

    let mut empty_tile_counter = applied_props.empty_count;
    'outer_for: for i in (0..grid_side_length_u32).rev() {
        for j in (0..grid_side_length_u32).rev() {
            let location = GridLocation::new(i as i32, j as i32);
            if solved_board.set_if_none(&location, Tile::new(TileType::Empty)) {
                empty_tile_counter -= 1;
                if empty_tile_counter == 0 {
                    break 'outer_for;
                }
            }
        }
    }

    for i in 0..grid_side_length_u32 {
        for j in 0..grid_side_length_u32 {
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set_if_none(&location, Tile::new(TileType::Numbered));
        }
    }

    solved_board.empty_locations_to_solved_default(applied_props.empty_count)?;
    solved_board.index_all_tile_types();
    solved_board.ignore_player_input = true;
    Ok(solved_board)
}

fn determine_wall_locations(
    applied_props: &BoardProperties,
) -> Result<Vec<GridLocation>, error_handler::BoardGenerationError> {
    let wall_count = applied_props.wall_count;
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut wall_spawn_locations = vec![];
    let mut possible_spawn_locations = vec![];
    let mut neighbor_count_grid =
        initialize_neighbor_count_grid(&mut possible_spawn_locations, grid_side_length);
    let mut grid_tree = neighbor_count_grid.get_spanning_tree(applied_props.tree_traveller_type);
    let mut grid_tree_iter = grid_tree.clone();

    for _ in 0..wall_count {
        let mut chosen_wall_location = GridLocation::default();
        let mut is_leaf;
        while !possible_spawn_locations.is_empty() {
            (chosen_wall_location, is_leaf) = match grid_tree_iter.next() {
                Some(tree_leaf) => (tree_leaf, true),
                None => (
                    util_functions::random_value(&possible_spawn_locations),
                    false,
                ),
            };

            // whether it's because the chosen location is illegal
            // or because we don't want to choose the same location twice
            // the chosen location has to be removed from the available ones
            let found_and_removed =
                remove_by_value(&chosen_wall_location, &mut possible_spawn_locations);

            // could be that the tree chose an illegal value
            if !found_and_removed {
                continue;
            } else {
                // if the leaf is valid, we want to remove it from its parents count
                // to allow the parent to eventually (hopefully)
                // become an available leaf
                grid_tree.decrease_parent_child_count(chosen_wall_location);
            }

            let chosen_tile_value = neighbor_count_grid.set_none_get_former(&chosen_wall_location);

            if is_leaf {
                //if the tile was determined by the MST, the graph is connected
                break;
            } else {
                //if wasn't a leaf, check if removing that tile keeps the graph connected,
                //if it doesn't - put the tile back, and reroll
                if neighbor_count_grid.is_connected_graph() {
                    break;
                } else {
                    //if the graph is not connected, we'll not remove this tile and thus it should be put back in place
                    neighbor_count_grid.set(&chosen_wall_location, chosen_tile_value.unwrap());
                }
            }
        }
        // if we didn't find an available spot
        if possible_spawn_locations.is_empty() {
            return Err(error_handler::BoardGenerationError::CouldntPlaceAllWalls);
        }

        // if the location was chosen
        wall_spawn_locations.push(chosen_wall_location);
        let neighbors_of_chosen_wall_location =
            neighbor_count_grid.get_all_direct_neighbor_locations(&chosen_wall_location);
        for neighbor in neighbors_of_chosen_wall_location {
            let neighbor_location = neighbor.1;
            let neighbor_value = neighbor_count_grid.get_mut(&neighbor_location).unwrap();
            *neighbor_value -= 1;

            // if a neigbor of the chosen location got to the threshold
            // we can't put a wall in its neighbor or it'll go below threshold
            if *neighbor_value == MIN_NEIGHBORS {
                forbid_spawn_in_neighbors_of_location(
                    &neighbor_location,
                    &mut possible_spawn_locations,
                    &neighbor_count_grid,
                );
            }
        }
    }
    Ok(wall_spawn_locations)
}

fn initialize_neighbor_count_grid(
    allowed_wall_spawn_locations: &mut Vec<GridLocation>,
    grid_side_length: u8,
) -> Grid<u8> {
    let mut neighbor_count_grid = Grid::new(grid_side_length);
    for inner_cell in neighbor_count_grid.all_locations_no_edges() {
        neighbor_count_grid.set(&inner_cell, 4);
        allowed_wall_spawn_locations.push(inner_cell);
    }
    for edge_tile_not_corner in neighbor_count_grid.edges_without_corners_locations() {
        neighbor_count_grid.set(&edge_tile_not_corner, 3);
        allowed_wall_spawn_locations.push(edge_tile_not_corner);
    }
    for corner in neighbor_count_grid.corner_locations() {
        neighbor_count_grid.set(&corner, 2);
        allowed_wall_spawn_locations.push(corner);
        if MIN_NEIGHBORS >= 2 {
            forbid_spawn_in_neighbors_of_location(
                &corner,
                allowed_wall_spawn_locations,
                &neighbor_count_grid,
            );
        }
    }
    neighbor_count_grid
}

fn forbid_spawn_in_neighbors_of_location(
    location: &GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &Grid<u8>,
) {
    for neighbor_to_forbid in neighbor_count_grid.get_all_direct_neighbor_locations(location) {
        util_functions::remove_by_value::<GridLocation>(
            &neighbor_to_forbid.1,
            possible_spawn_locations,
        );
    }
}

fn spawn_walls_in_locations(locations: Vec<GridLocation>, board: &mut TileTypeBoard) {
    for location in locations {
        board.set(&location, Tile::new(TileType::Wall));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity_bfs_tree() {
        const ATTEMPT_COUNT: u8 = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::BFS,
            ..Default::default()
        };
        for _ in 0..ATTEMPT_COUNT {
            let solved_board = generate_solved_board(&board_props).unwrap();
            assert!(solved_board.grid.is_connected_graph());
        }
    }

    #[test]
    fn test_connectivity_dfs_tree() {
        const ATTEMPT_COUNT: u8 = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::DFS,
            ..Default::default()
        };
        for _ in 0..ATTEMPT_COUNT {
            let solved_board = generate_solved_board(&board_props).unwrap();
            assert!(solved_board.grid.is_connected_graph());
        }
    }
}
