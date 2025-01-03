use crate::prelude::*;

/// mustn't be more than 2 as there will always be a corner
/// shouldn't be less than 1, or we might get useless spaces
const MIN_NEIGHBORS: u8 = 2;

const SOLVED_BOARD_WITH_WALLS_MAX_GENERATION_ATTEMPTS: u8 = 20;

struct WallLocationChosen(pub bool);

#[derive(Component)]
pub struct SolvedBoard;

pub struct SolvedBoardPlugin;

impl Plugin for SolvedBoardPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                OnEnter(GameState::PendingSolvedBoardGeneration),
                generate_solved_board
            );
    }
}

fn generate_solved_board(
    mut generation_error_event_writer: EventWriter<ShowGenerationError>,
    mut solved_board_query: Query<&mut TileBoard, With<SolvedBoard>>,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut game_state: ResMut<NextState<GameState>>,
){
    let applied_props = applied_board_props_query.single();
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut new_solved_board = TileBoard::new(grid_side_length);

    match generate_solved_board_inner(
        applied_props,
        &mut new_solved_board
    ) {
        Ok(_) => {
            *solved_board_query.single_mut() = new_solved_board;
            game_state.set(GameState::SolvedBoardGenerated);
        },
        Err(board_gen_error) => {
            generation_error_event_writer.send(ShowGenerationError(board_gen_error));
            game_state.set(GameState::Regular);
        }
    }
}

pub fn generate_solved_board_inner(
    applied_props: &BoardProperties,
    solved_board: &mut TileBoard,
) -> Result<(), BoardGenerationError>
{
    let mut new_board_wall_locations= vec![];
    let grid_side_length_u32 = applied_props.size.to_grid_side_length() as u32;

    if applied_props.wall_count > 0 {
        determine_wall_locations(
            applied_props,
            &mut new_board_wall_locations
        )?;
    }

    empty_tile_board_to_solved(
        solved_board,
        new_board_wall_locations,
        applied_props,
        grid_side_length_u32
    )
}

pub fn empty_tile_board_to_solved(
    solved_board: &mut TileBoard,
    wall_locations: Vec<GridLocation>,
    applied_props: &BoardProperties,
    grid_side_length: u32
)
-> Result<(), BoardGenerationError>
{
    solved_board.spawn_walls_in_locations(&wall_locations)?;
    solved_board.spawn_empty_tiles(applied_props, &grid_side_length)?;
    solved_board.spawn_numbered_uninitialized_tiles(&grid_side_length)?;

    solved_board.empty_locations_to_solved_default(applied_props.empty_count)?;
    solved_board.index_all_tile_types();
    
    Ok(())
}

fn determine_wall_locations(
    applied_props: &BoardProperties,
    wall_locations: &mut Vec<GridLocation>,
) -> Result<(), BoardGenerationError>
{
    let mut wall_location_determination_attempt = 1;
    let mut wall_location_determination_result =
        determine_wall_locations_inner(applied_props);
    while wall_location_determination_attempt < SOLVED_BOARD_WITH_WALLS_MAX_GENERATION_ATTEMPTS
        && wall_location_determination_result.is_err()
    {
        wall_location_determination_result = determine_wall_locations_inner(applied_props);
        wall_location_determination_attempt += 1;
    }
    match wall_location_determination_result{
        Ok(just_determined_wall_locations) => {
            *wall_locations = just_determined_wall_locations;
        },
        Err(wall_location_finding_error) => return Err(wall_location_finding_error)
    }
    Ok(())
}

fn determine_wall_locations_inner(
    applied_props: &BoardProperties,
) -> Result<Vec<GridLocation>, BoardGenerationError>
{
    let wall_count = applied_props.wall_count;
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut wall_spawn_locations = vec![];
    let mut possible_spawn_locations = vec![];
    let neighbor_count_grid_result =
        initialize_neighbor_count_grid(&mut possible_spawn_locations, grid_side_length);
    let mut neighbor_count_grid = neighbor_count_grid_result?;
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
            neighbor_count_grid.get_all_initialized_neighbor_locations(&chosen_wall_location);
        for neighbor in neighbors_of_chosen_wall_location {
            let neighbor_location = neighbor.1;
            let neighbor_value = neighbor_count_grid.get_mut(&neighbor_location).unwrap().unwrap();
            *neighbor_value -= 1;

            // if a neighbor of the chosen location got to the threshold
            // we can't put walls near it, or it'll go below threshold
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
    let is_chosen_wall_location_leaf_in_grid_tree;
    (*chosen_wall_location, is_chosen_wall_location_leaf_in_grid_tree) = match grid_tree_iter.next() {
        Some(tree_leaf) => (tree_leaf, true),
        None => (
            random_value(possible_spawn_locations),
            false,
        )
    };

    let wall_placement_validation = validate_wall_placement::<u8>(
        chosen_wall_location,
        &is_chosen_wall_location_leaf_in_grid_tree,
        possible_spawn_locations,
        neighbor_count_grid,
        grid_tree,
    )?;

    handle_wall_validation_object(
        wall_placement_validation,
        chosen_wall_location,
        possible_spawn_locations,
        neighbor_count_grid
    )
}

fn handle_wall_validation_object(
    wall_placement_validation: BoardIsValid<u8>,
    chosen_wall_location: &mut GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &mut Grid<u8>,
) -> Result<WallLocationChosen, BoardGenerationError>
{
    let board_is_valid = 
        wall_placement_validation.board_is_valid;
    let index_to_remove_from_possible_walls = 
        wall_placement_validation.index_to_remove_from_possible_walls;
    let cell_value_in_checked_location_result = 
        wall_placement_validation.cell_value_in_checked_location_result?;

    if ! board_is_valid {
        put_cell_back_in_place(
            chosen_wall_location,
            &cell_value_in_checked_location_result,
            neighbor_count_grid
        )?;
    }

    //whether because it's illegal or because we don't want to choose the same place twice
    if let Some(found_index) = index_to_remove_from_possible_walls {
        possible_spawn_locations.swap_remove(found_index);
    }

    Ok(WallLocationChosen(board_is_valid))
}

fn put_cell_back_in_place(
    chosen_wall_location_ref: &GridLocation,
    optional_chosen_tile_value: &Option<u8>,
    neighbor_count_grid_ref_mut: &mut Grid<u8>
) 
-> Result<(), BoardGenerationError>
{
    if let Some(chosen_tile_value) = optional_chosen_tile_value{
        Ok(neighbor_count_grid_ref_mut.set(chosen_wall_location_ref, *chosen_tile_value)?)
    }else{
        Ok(())
    }
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
    for neighbor_to_forbid in neighbor_count_grid.get_all_initialized_neighbor_locations(location) {
        remove_by_value::<GridLocation>(
            &neighbor_to_forbid.1,
            possible_spawn_locations,
        );
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connectivity_bfs_tree() {
        let mut app = App::new();
        app.add_systems(Update, test_connectivity_bfs_tree_inner);
        app.update();
    }

    fn test_connectivity_bfs_tree_inner() {
        const ATTEMPT_COUNT: usize = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::BFS,
            ..Default::default()
        };
        let mut tile_board = TileBoard::new(board_props.size.to_grid_side_length());
        for _ in 0..ATTEMPT_COUNT {
            let _ = generate_solved_board_inner(
                &board_props, 
                &mut tile_board
            );

            assert!(tile_board.grid.is_connected_graph());
        }
    }

    #[test]
    fn test_connectivity_dfs_tree() {
        let mut app = App::new();
        app.add_systems(Update, test_connectivity_dfs_tree_inner);
        app.update();
    }

    fn test_connectivity_dfs_tree_inner() {
        const ATTEMPT_COUNT: usize = 42;
        const WALL_COUNT_FOR_TEST: u8 = 2;
        let board_props: BoardProperties = BoardProperties {
            size: BoardSize::Giant,
            wall_count: WALL_COUNT_FOR_TEST,
            tree_traveller_type: GridTravellerType::DFS,
            ..Default::default()
        };
        let mut tile_board = TileBoard::new(board_props.size.to_grid_side_length());
        for _ in 0..ATTEMPT_COUNT {
            let _ = generate_solved_board_inner(
                &board_props,
                &mut tile_board
            );
            assert!(tile_board.grid.is_connected_graph());
        }
    }
}