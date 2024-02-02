use crate::{logic::data_structure::util_functions, output::error_handler, prelude::*, costume_event::{board_set_event, ui_event}};
use crate::logic::states::game_state;

/// mustn't be more than 2 as there will always be a corner
/// shouldn't be less than 1 or we might get useless spaces
const MIN_NEIGHBORS: u8 = 2;

pub struct SolvedBoardPlugin;

impl Plugin for SolvedBoardPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::PendingSolvedBoardGen),
                generate_solved_board
            );
    }
}

fn generate_solved_board(
    mut generation_error_event_writer: EventWriter<ui_event::ShowGenerationError>,
    mut solved_board_query: Query<&mut TileBoard, With<SolvedBoard>>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut db_manager: ResMut<DataBaseManager>,
    mut game_state: ResMut<NextState<GameState>>,
){
    match generate_solved_board_inner(
        applied_board_props_query.single(),
        db_manager.as_mut()
    ) {
        Ok(board) => {
            *solved_board_query.single_mut() = board;
            game_state.set(GameState::SolvedBoardGenerated);
        },
        Err(error) => {
            generation_error_event_writer.send(ui_event::ShowGenerationError(error));
            return;
        }
    }
}

pub fn generate_solved_board_inner(
    applied_props: &BoardProperties,
    db_manager: &mut DataBaseManager
) -> Result<TileBoard, BoardGenerationError> {
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut solved_board = TileBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;
    let mut wall_locations = vec![];

    if applied_props.wall_count > 0 {
        wall_locations = determine_wall_locations(applied_props)?;
        wrap_if_error
            (&spawn_walls_in_locations(&wall_locations, &mut solved_board))?;
    }

    db_manager.insert_layout(SavedLayout{
        board_propes: *applied_props,
        wall_locations
    });

    let mut empty_tile_counter = applied_props.empty_count;
    'outer_for: for i in (0..grid_side_length_u32).rev() {
        for j in (0..grid_side_length_u32).rev() {
            let location = GridLocation::new(i as i32, j as i32);
            if wrap_if_error(&solved_board.tiletype_in_location(&location))?.is_none() {
                wrap_if_error
                    (&solved_board.set(&location, Tile::new(TileType::Empty)))?;
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
            if wrap_if_error(&solved_board.tiletype_in_location(&location))?.is_none() {
                wrap_if_error
                    (&solved_board.set(&location, Tile::new(TileType::Numbered)))?
            }
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
    let neighbor_count_grid_result =
        initialize_neighbor_count_grid(&mut possible_spawn_locations, grid_side_length);
    let mut neighbor_count_grid =
        wrap_if_error_owned(neighbor_count_grid_result)?;
    let grid_tree_result = 
        neighbor_count_grid.get_spanning_tree(applied_props.tree_traveller_type);
    let mut grid_tree;
    let mut grid_tree_iter;
    match grid_tree_result{
        Ok(valid_grid_tree) => {
            grid_tree = valid_grid_tree;
            grid_tree_iter = grid_tree.clone();
        },
        Err(tree_error) => return Err(error_handler::BoardGenerationError::GridTreeError(tree_error))
    }
    

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
            let removed_item =
                remove_by_value(&chosen_wall_location, &mut possible_spawn_locations);

            // could be that the tree chose an illegal value
            if removed_item.is_none() {
                continue;
            } else {
                // if the leaf is valid, we want to remove it from its parents count
                // to allow the parent to eventually (hopefully)
                // become an available leaf
                if let Err(grid_tree_error) = grid_tree.decrease_parent_child_count(chosen_wall_location){
                    return Err(error_handler::BoardGenerationError::GridTreeError(grid_tree_error));
                }
            }

            let chosen_tile_value_result: Result<Option<u8>, GridError> = 
                neighbor_count_grid.set_none_get_former(&chosen_wall_location);

            match neighbor_count_grid.all_nodes_in_cycles() {
                Err(data_struct_error) => {
                    return Err(error_handler::BoardGenerationError::CircleCheckError(data_struct_error));
                },
                Ok(all_in_cycles) => {
                    if !all_in_cycles{
                        put_cell_back_in_place(
                            &chosen_wall_location,
                            &chosen_tile_value_result,
                            &mut neighbor_count_grid
                        )?;
                        continue;
                    }
                }
            }

            //check (or don't) connectivity
            if is_leaf {
                //if the tile was determined by the MST, the graph is connected
                break;
            } else {
                //if wasn't a leaf, check if removing that tile keeps the graph connected,
                //if it doesn't - put the tile back, and reroll
                if neighbor_count_grid.is_connected_graph() {
                    break;
                } else {
                    put_cell_back_in_place(
                        &chosen_wall_location,
                        &chosen_tile_value_result,
                        &mut neighbor_count_grid
                    )?;
                    continue;
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
            neighbor_count_grid.get_all_occupied_neighbor_locations(&chosen_wall_location);
        for neighbor in neighbors_of_chosen_wall_location {
            let neighbor_location = neighbor.1;
            let neighbor_value = neighbor_count_grid.get_mut(&neighbor_location).unwrap().unwrap();
            *neighbor_value -= 1;

            // if a neighbor of the chosen location got to the threshold
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

fn put_cell_back_in_place(
    chosen_wall_location_ref: &GridLocation,
    chosen_tile_value_result_ref: &Result<Option<u8>, GridError>,
    neighbor_count_grid_ref_mut: &mut Grid<u8>
) 
-> Result<(), error_handler::BoardGenerationError>
{
    let chosen_tile_value= 
        wrap_if_error(chosen_tile_value_result_ref)?;
    Ok(
        wrap_if_error
            (&neighbor_count_grid_ref_mut.set
                (chosen_wall_location_ref, chosen_tile_value.unwrap()))?
    )
}

fn initialize_neighbor_count_grid(
    allowed_wall_spawn_locations: &mut Vec<GridLocation>,
    grid_side_length: u8,
) -> Result<Grid<u8>, error_handler::GridError> {
    let mut neighbor_count_grid = Grid::new(grid_side_length);
    for inner_cell in neighbor_count_grid.all_locations_no_edges() {
        neighbor_count_grid.set(&inner_cell, 4)?;
        allowed_wall_spawn_locations.push(inner_cell);
    }
    for edge_tile_not_corner in neighbor_count_grid.edges_without_corners_locations() {
        neighbor_count_grid.set(&edge_tile_not_corner, 3)?;
        allowed_wall_spawn_locations.push(edge_tile_not_corner);
    }
    for corner in neighbor_count_grid.corner_locations() {
        neighbor_count_grid.set(&corner, 2)?;
        allowed_wall_spawn_locations.push(corner);
        if MIN_NEIGHBORS >= 2 {
            forbid_spawn_in_neighbors_of_location(
                &corner,
                allowed_wall_spawn_locations,
                &neighbor_count_grid,
            );
        }
    }
    Ok(neighbor_count_grid)
}

fn forbid_spawn_in_neighbors_of_location(
    location: &GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &Grid<u8>,
) {
    for neighbor_to_forbid in neighbor_count_grid.get_all_occupied_neighbor_locations(location) {
        util_functions::remove_by_value::<GridLocation>(
            &neighbor_to_forbid.1,
            possible_spawn_locations,
        );
    }
}

fn spawn_walls_in_locations(locations: &Vec<GridLocation>, board: &mut TileBoard)
-> Result<(), error_handler::GridError>
{
    for location in locations {
        board.set(location, Tile::new(TileType::Wall))?;
    }
    Ok(())
}

fn wrap_if_error<T: Copy>(result: &Result<T, error_handler::GridError>) 
-> Result<T, error_handler::BoardGenerationError>{
    match result {
        Err(grid_error) => {
            Err(error_handler::BoardGenerationError::GridError(*grid_error))
        },
        Ok(value) => Ok(*value)
    }
}

fn wrap_if_error_owned<T>(result: Result<T, GridError>)
    -> Result<T, error_handler::BoardGenerationError>{
    match result {
        Err(grid_error) => {
            Err(error_handler::BoardGenerationError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity_bfs_tree() {
        const ATTEMPT_COUNT: usize = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::BFS,
            ..Default::default()
        };
        let mut dummy_db_manager = DataBaseManager::default();
        for _ in 0..ATTEMPT_COUNT {
            let solved_board = 
                generate_solved_board_inner(
                    &board_props,
                    &mut dummy_db_manager
                ).unwrap();
            assert!(solved_board.grid.is_connected_graph());
        }
        assert_eq!(dummy_db_manager.get_saved_layouts_ref().len(), ATTEMPT_COUNT);
    }

    #[test]
    fn test_connectivity_dfs_tree() {
        const ATTEMPT_COUNT: usize = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::DFS,
            ..Default::default()
        };
        let mut dummy_db_manager = DataBaseManager::default();
        for _ in 0..ATTEMPT_COUNT {
            let solved_board = 
                generate_solved_board_inner(
                    &board_props,
                    &mut dummy_db_manager
                ).unwrap();
            assert!(solved_board.grid.is_connected_graph());
        }
        assert_eq!(dummy_db_manager.get_saved_layouts_ref().len(), ATTEMPT_COUNT);
    }
}
