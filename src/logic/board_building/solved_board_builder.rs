use crate::prelude::*;

/// mustn't be more than 2 as there will always be a corner
/// shouldn't be less than 1 or we might get useless spaces
const MIN_NEIGHBORS: u8 = 2;

const SOLVED_BOARD_WITH_WALLS_MAX_GENERATION_ATTEMPTS: u8 = 20;

struct WallLocationChosen(pub bool);
struct LocationFoundInPossibleLocations(pub bool);
struct AllTilesStillInCycles(pub bool);
struct GraphIsStillConnected(pub bool);

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
    mut generation_error_event_writer: EventWriter<ShowGenerationError>,
    mut solved_board_query: Query<&mut TileBoard, With<SolvedBoard>>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut write_to_db_event_writer: EventWriter<SaveToDB>
){
    match generate_solved_board_inner(
        applied_board_props_query.single(),
        &mut write_to_db_event_writer
    ) {
        Ok(board) => {
            *solved_board_query.single_mut() = board;
            game_state.set(GameState::SolvedBoardGenerated);
        },
        Err(error) => {
            generation_error_event_writer.send(ShowGenerationError(error));
            game_state.set(GameState::Regular);
        }
    }
}

pub fn generate_solved_board_inner(
    applied_props: &BoardProperties,
    write_to_db_event_writer: &mut EventWriter<SaveToDB>
) -> Result<TileBoard, BoardGenerationError> {
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut solved_board = TileBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;
    let mut wall_locations = vec![];

    if applied_props.wall_count > 0 {
        let mut wall_location_determination_attempt = 1;
        let mut wall_location_determination_result =
            determine_wall_locations(applied_props);
        while wall_location_determination_attempt < SOLVED_BOARD_WITH_WALLS_MAX_GENERATION_ATTEMPTS
            && wall_location_determination_result.is_err()
        {
            wall_location_determination_result = determine_wall_locations(applied_props);
            wall_location_determination_attempt += 1;
        }
        match wall_location_determination_result{
            Ok(just_determined_wall_locations) => {
                wall_locations = just_determined_wall_locations;
            },
            Err(wall_location_finding_error) => return Err(wall_location_finding_error)
        }
        wrap_if_error
            (&spawn_walls_in_locations(&wall_locations, &mut solved_board))?;
    }

    write_to_db_event_writer.send(SaveToDB(DomainBoard{
        board_props: *applied_props,
        wall_locations
    }));

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
) -> Result<Vec<GridLocation>, BoardGenerationError> {
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
        Err(tree_error) => return Err(BoardGenerationError::GridTreeError(tree_error))
    }

    for _ in 0..wall_count {
        determine_wall_location(
            &mut wall_spawn_locations,
            &mut possible_spawn_locations,
            &mut neighbor_count_grid,
            &mut grid_tree,
            &mut grid_tree_iter
        )?;
    }

    Ok(wall_spawn_locations)
}

fn determine_wall_location(
    wall_spawn_locations: &mut Vec<GridLocation>,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &mut Grid<u8>,
    grid_tree: &mut GridTree,
    grid_tree_iter: &mut GridTree,
) -> Result<(), BoardGenerationError>
{
    let mut chosen_wall_location = GridLocation::default();
    let mut wall_location_found = false;
    while !wall_location_found && !possible_spawn_locations.is_empty() {
        wall_location_found = roll_and_validate_wall_location(
            &mut chosen_wall_location,
            possible_spawn_locations,
            neighbor_count_grid,
            grid_tree,
            grid_tree_iter,
        )?.0;
    }

    if wall_location_found {
        wall_spawn_locations.push(chosen_wall_location);
        let neighbors_of_chosen_wall_location =
            neighbor_count_grid.get_all_occupied_neighbor_locations(&chosen_wall_location);
        for neighbor in neighbors_of_chosen_wall_location {
            let neighbor_location = neighbor.1;
            let neighbor_value = neighbor_count_grid.get_mut(&neighbor_location).unwrap().unwrap();
            *neighbor_value -= 1;

            // if a neighbor of the chosen location got to the threshold
            // we can't put walls near it or it'll go below threshold
            if *neighbor_value == MIN_NEIGHBORS {
                forbid_spawn_in_neighbors_of_location(
                    &neighbor_location,
                    possible_spawn_locations,
                    neighbor_count_grid,
                );
            }
        }
        Ok(())
    }else{
        Err(BoardGenerationError::CouldntPlaceAllWalls)
    }
}

fn roll_and_validate_wall_location(
    chosen_wall_location: &mut GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &mut Grid<u8>,
    grid_tree: &mut GridTree,
    grid_tree_iter: &mut GridTree,
) -> Result<WallLocationChosen, BoardGenerationError>
{
    let is_leaf;
    (*chosen_wall_location, is_leaf) = match grid_tree_iter.next() {
        Some(tree_leaf) => (tree_leaf, true),
        None => (
            random_value(possible_spawn_locations),
            false,
        )
    };

    match remove_wall_location_from_possible_locations(
        chosen_wall_location,
        possible_spawn_locations,
        grid_tree
    )?
    {
        LocationFoundInPossibleLocations(false) => {
            Ok(WallLocationChosen(false))
        },
        LocationFoundInPossibleLocations(true) => {
            let chosen_tile_value_result: Result<Option<u8>, GridError> =
                neighbor_count_grid.set_none_get_former(chosen_wall_location);



            // info!("checking cycles for wall in {:?}", chosen_wall_location);


            match ensure_all_walls_in_cycle(
                chosen_wall_location,
                &chosen_tile_value_result,
                neighbor_count_grid
            )?
            {
                AllTilesStillInCycles(false) => {
                    Ok(WallLocationChosen(false))
                },
                AllTilesStillInCycles(true) => {
                    match ensure_connectivity(
                        is_leaf,
                        chosen_wall_location,
                        &chosen_tile_value_result,
                        neighbor_count_grid
                    )?
                    {
                        GraphIsStillConnected(is_graph_still_connected) => {
                            Ok(WallLocationChosen(is_graph_still_connected))
                        }
                    }
                }
            }
        }
    }
}

/// whether because it's illegal or because we don't want to chose the same place twice
fn remove_wall_location_from_possible_locations(
    chosen_wall_location: &mut GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    grid_tree: &mut GridTree,
) -> Result<LocationFoundInPossibleLocations, BoardGenerationError>
{
    let location_removed_from_possible_wall_locations =
        remove_by_value(chosen_wall_location, possible_spawn_locations);

    // could that the place chosen by the tree wasn't a valid location (say, too few neighbors)
    if location_removed_from_possible_wall_locations.is_none() {
        Ok(LocationFoundInPossibleLocations(false))
    } else {
        // if the leaf is valid, we want to remove it from its parent's count
        // to allow the parent to eventually (hopefully) become an available leaf
        if let Err(grid_tree_error) = grid_tree.decrease_parent_child_count(*chosen_wall_location){
            Err(BoardGenerationError::GridTreeError(grid_tree_error))
        }else{
            Ok(LocationFoundInPossibleLocations(true))
        }
    }
}

fn ensure_all_walls_in_cycle(
    chosen_wall_location: &GridLocation,
    chosen_tile_value_result: &Result<Option<u8>, GridError>,
    neighbor_count_grid: &mut Grid<u8>,
) -> Result<AllTilesStillInCycles, BoardGenerationError>
{
    match neighbor_count_grid.all_nodes_in_cycles() {
        Err(data_struct_error) => {
            Err(BoardGenerationError::CircleCheckError(data_struct_error))
        },
        Ok(all_in_cycles) => {
            if all_in_cycles{
                Ok(AllTilesStillInCycles(true))
            }else{
                put_cell_back_in_place(
                    chosen_wall_location,
                    chosen_tile_value_result,
                    neighbor_count_grid
                )?;
                Ok(AllTilesStillInCycles(false))
            }
        }
    }
}

fn ensure_connectivity(
    is_leaf: bool,
    chosen_wall_location: &GridLocation,
    chosen_tile_value_result: &Result<Option<u8>, GridError>,
    neighbor_count_grid: &mut Grid<u8>,
) -> Result<GraphIsStillConnected, BoardGenerationError>
{
    if is_leaf {
        //if the tile was determined by the MST, the graph is connected
        Ok(GraphIsStillConnected(true))
    } else {
        //if wasn't a leaf, check if removing that tile keeps the graph connected,
        //if it doesn't - put the tile back, and reroll
        if neighbor_count_grid.is_connected_graph() {
            Ok(GraphIsStillConnected(true))
        } else {
            put_cell_back_in_place(
                chosen_wall_location,
                chosen_tile_value_result,
                neighbor_count_grid
            )?;
            Ok(GraphIsStillConnected(false))
        }
    }
}

fn put_cell_back_in_place(
    chosen_wall_location_ref: &GridLocation,
    chosen_tile_value_result_ref: &Result<Option<u8>, GridError>,
    neighbor_count_grid_ref_mut: &mut Grid<u8>
) 
-> Result<(), BoardGenerationError>
{
    let chosen_tile_value= 
        wrap_if_error(chosen_tile_value_result_ref)?;
        wrap_if_error
            (&neighbor_count_grid_ref_mut.set
                (chosen_wall_location_ref, chosen_tile_value.unwrap()))
}

fn initialize_neighbor_count_grid(
    allowed_wall_spawn_locations: &mut Vec<GridLocation>,
    grid_side_length: u8,
) -> Result<Grid<u8>, GridError> {
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
        remove_by_value::<GridLocation>(
            &neighbor_to_forbid.1,
            possible_spawn_locations,
        );
    }
}

fn spawn_walls_in_locations(locations: &Vec<GridLocation>, board: &mut TileBoard)
-> Result<(), GridError>
{
    for location in locations {
        board.set(location, Tile::new(TileType::Wall))?;
    }
    Ok(())
}

fn wrap_if_error<T: Copy>(result: &Result<T, GridError>)
-> Result<T, BoardGenerationError>{
    match result {
        Err(grid_error) => {
            Err(BoardGenerationError::GridError(*grid_error))
        },
        Ok(value) => Ok(*value)
    }
}

fn wrap_if_error_owned<T>(result: Result<T, GridError>)
    -> Result<T, BoardGenerationError>{
    match result {
        Err(grid_error) => {
            Err(BoardGenerationError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity_bfs_tree() {
        let mut app = App::new();
        app.add_event::<SaveToDB>()
            .add_systems(Update, test_connectivity_bfs_tree_inner);
        app.update();
    }

    fn test_connectivity_bfs_tree_inner(
        mut event_writer: EventWriter<SaveToDB>,
    ) {
        const ATTEMPT_COUNT: usize = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::BFS,
            ..Default::default()
        };
        for _ in 0..ATTEMPT_COUNT {
            let solved_board =
                generate_solved_board_inner(
                    &board_props,
                    &mut event_writer
                ).unwrap();
            assert!(solved_board.grid.is_connected_graph());
        }
    }

    #[test]
    fn test_connectivity_dfs_tree() {
        let mut app = App::new();
        app.add_event::<SaveToDB>()
            .add_systems(Update, test_connectivity_dfs_tree_inner);
        app.update();
    }

    fn test_connectivity_dfs_tree_inner(
        mut event_writer: EventWriter<SaveToDB>,
    ) {
        const ATTEMPT_COUNT: usize = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::DFS,
            ..Default::default()
        };
        for _ in 0..ATTEMPT_COUNT {
            let solved_board =
                generate_solved_board_inner(
                    &board_props,
                    &mut event_writer
                ).unwrap();
            assert!(solved_board.grid.is_connected_graph());
        }
    }
}