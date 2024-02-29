use crate::prelude::*;

pub struct BoardIsValid<T: Clone> {
    pub board_is_valid: bool,
    pub index_to_remove_from_possible_walls: Option<usize>,
    pub cell_value_in_checked_location_result: Result<Option<T>, GridError>
}

struct LocationFoundInPossibleLocations {
    pub found: bool,
    pub index: Option<usize>
}
struct AllTilesStillInCycles(pub bool);

pub fn validate_wall_placement<T: Clone>(
    chosen_wall_location: &mut GridLocation,
    is_chosen_wall_location_leaf_in_grid_tree: &bool,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &mut Grid<T>,
    grid_tree: &mut GridTree,
) -> Result<BoardIsValid<T>, BoardGenerationError>
{
    let valid_neighbor_count_result =
    chosen_location_found_in_valid_locations_list(
        chosen_wall_location,
        possible_spawn_locations,
        grid_tree
    )?;

    let chosen_tile_value_result: Result<Option<T>, GridError> =
        neighbor_count_grid.set_none_get_former(chosen_wall_location);
    
    let board_is_valid =
        match valid_neighbor_count_result.found{
            false => {
                false
            },
            true => {
                match ensure_all_walls_in_cycle(
                    neighbor_count_grid
                )?
                {
                    AllTilesStillInCycles(false) => {
                        false
                    },
                    AllTilesStillInCycles(true) => { 
                        *is_chosen_wall_location_leaf_in_grid_tree || neighbor_count_grid.is_connected_graph()
                    }
                }
            }
        };

    Ok(BoardIsValid{
        board_is_valid,
        index_to_remove_from_possible_walls: valid_neighbor_count_result.index,
        cell_value_in_checked_location_result: chosen_tile_value_result
    })
}

fn chosen_location_found_in_valid_locations_list(
    chosen_wall_location: &mut GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    grid_tree: &mut GridTree,
) -> Result<LocationFoundInPossibleLocations, BoardGenerationError>
{
    let index_of_chosen_location_in_possible_ones =
        item_to_index(chosen_wall_location, possible_spawn_locations);

    // could that the place chosen by the tree wasn't a valid location (say, too few neighbors)
    if index_of_chosen_location_in_possible_ones.is_none() {
        Ok(LocationFoundInPossibleLocations {
            found: false,
            index: None
        })
    } else {
        // if the leaf is valid, we want to remove it from its parent's count
        // to allow the parent to eventually (hopefully) become an available leaf
        if let Err(grid_tree_error) = grid_tree.decrease_parent_child_count(*chosen_wall_location){
            Err(BoardGenerationError::GridTreeError(grid_tree_error))
        }else{
            Ok(LocationFoundInPossibleLocations {
                found: true,
                index: index_of_chosen_location_in_possible_ones
            })
        }
    }
}

fn ensure_all_walls_in_cycle<T: Clone>(
    grid_to_check: &mut Grid<T>,
) -> Result<AllTilesStillInCycles, BoardGenerationError>
{
    match grid_to_check.all_nodes_in_cycles() {
        Err(data_struct_error) => {
            Err(BoardGenerationError::CircleCheckError(data_struct_error))
        },
        Ok(all_in_cycles) => {
            if all_in_cycles{
                Ok(AllTilesStillInCycles(true))
            }else{
                Ok(AllTilesStillInCycles(false))
            }
        }
    }
}