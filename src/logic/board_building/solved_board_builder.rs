use crate::{prelude::*, output::error_handler};
use rand::Rng;

const MIN_NEIGHBORS: u8 = 2;

pub fn generate_solved_board(applied_props: &BoardProperties) -> Result<TileTypeBoard, BoardGenerationError>{
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;

    if applied_props.wall_count > 0 {
        let wall_locations 
            = determine_wall_locations(applied_props.wall_count, grid_side_length)?;
        spawn_walls_in_locations(
            wall_locations, 
            &mut solved_board
        );
    }

    let mut empty_tile_counter = applied_props.empty_count;
    'outer_for: for i in (0..grid_side_length_u32).rev(){
        for j in (0..grid_side_length_u32).rev(){
            let location = GridLocation::new(i as i32, j as i32);
            if solved_board.set_if_none(&location, Tile::new(TileType::Empty)){
                empty_tile_counter -= 1 ;
                if empty_tile_counter == 0 {
                    break 'outer_for;
                }
            }
        }
    }

    for i in 0..grid_side_length_u32{
        for j in 0..grid_side_length_u32{
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set_if_none(&location, Tile::new(TileType::Numbered));
        }
    }

    solved_board.empty_locations_to_solved_default(applied_props.empty_count)?;
    solved_board.index_all_tile_types();
    solved_board.ignore_player_input=true;
    Ok(solved_board)
}


fn determine_wall_locations(wall_count: u8, grid_side_length: u8) 
-> Result<Vec<GridLocation>, error_handler::BoardGenerationError>
{
    let mut rng = rand::thread_rng();
    let mut wall_spawn_locations = vec![];
    let mut possible_spawn_locations = vec![];
    let mut neighbor_count_grid = initialize_neighbor_count_grid(
        &mut possible_spawn_locations, 
        grid_side_length
    );

    for _ in 0..wall_count{
        let mut chosen_wall_location = GridLocation::default();
        while ! possible_spawn_locations.is_empty(){
            let chosen_wall_location_index = rng.gen_range(0..possible_spawn_locations.len());
            chosen_wall_location = possible_spawn_locations[chosen_wall_location_index];
    
            // whether it's because the chosen location is illegal 
            // or because we don't want to choose the same location twice
            // the chosen location has to be removed from the available ones
            remove_by_value(
                &chosen_wall_location, 
                &mut possible_spawn_locations
            );

            //check if removing that tile keeps the graph connected, 
            //if not - put it back, and reroll
            let chosen_tile_value 
                = neighbor_count_grid.set_none_get_former(&chosen_wall_location);
            if neighbor_count_grid.is_connected_graph(){
                break;
            }else{
                //if the graph is not connected, we'll not remove this tile and thus it should be put back in place
                neighbor_count_grid.set(&chosen_wall_location, chosen_tile_value.unwrap());
            }
        }
        // if we didn't find an available spot
        if possible_spawn_locations.is_empty() {
            return Err(error_handler::BoardGenerationError::CouldntPlaceAllWalls);
        }

        // if the location was chosen
        wall_spawn_locations.push(chosen_wall_location);
        let neighbors_of_chosen_wall_location 
            = neighbor_count_grid.get_all_direct_neighbor_locations(&chosen_wall_location);     
        for neighbor in neighbors_of_chosen_wall_location
        {
            let neighbor_location = neighbor.1;
            let neighbor_value = neighbor_count_grid.get_mut(&neighbor_location).unwrap();
            *neighbor_value -= 1;

            // if a neigbor of the chosen location got to the threshold
            // we can't put a wall in its neighbor or it'll go below threshold
            if *neighbor_value == MIN_NEIGHBORS{
                forbid_spawn_in_neighbors_of_location(
                    &neighbor_location,
                    &mut possible_spawn_locations,
                    &neighbor_count_grid
                );
            }
        }
    }
    Ok(wall_spawn_locations)
}

fn initialize_neighbor_count_grid(
    allowed_wall_spawn_locations: &mut Vec<GridLocation>,
    grid_side_length: u8
) -> Grid<u8>
{
    let mut neighbor_count_grid = Grid::new(grid_side_length);
    for inner_cell in neighbor_count_grid.all_locations_no_edges(){
        neighbor_count_grid.set(&inner_cell, 4);
        allowed_wall_spawn_locations.push(inner_cell);
    }
    for edge_tile_not_corner in neighbor_count_grid.edges_without_corners_locations(){
        neighbor_count_grid.set(&edge_tile_not_corner, 3);
        allowed_wall_spawn_locations.push(edge_tile_not_corner);
        if MIN_NEIGHBORS == 3 {
            forbid_spawn_in_neighbors_of_location(
                &edge_tile_not_corner,
                allowed_wall_spawn_locations,
                &neighbor_count_grid
            );
        }
    }
    for corner in neighbor_count_grid.corner_locations(){
        neighbor_count_grid.set(&corner, 2);
        allowed_wall_spawn_locations.push(corner);
        if MIN_NEIGHBORS >= 2 {
            forbid_spawn_in_neighbors_of_location(
                &corner,
                allowed_wall_spawn_locations,
                &neighbor_count_grid
            );
        }
    }
    neighbor_count_grid
}

fn forbid_spawn_in_neighbors_of_location(
    location: &GridLocation,
    possible_spawn_locations: &mut Vec<GridLocation>,
    neighbor_count_grid: &Grid<u8>
){
    for neighbor_to_forbid in 
        neighbor_count_grid.get_all_direct_neighbor_locations(&location)
    {
        remove_by_value(
            &neighbor_to_forbid.1, 
            possible_spawn_locations
        );
    }
}

fn remove_by_value(location_to_forbid: &GridLocation, possible_spawn_locations: &mut Vec<GridLocation>){
    let optional_index_to_remove = possible_spawn_locations.iter()
        .position(|x| *x == *location_to_forbid);
    //could be that it was removed before by a different neighbor
    if let Some(index_to_remove) =  optional_index_to_remove {
        possible_spawn_locations.swap_remove(index_to_remove);
    }
}

fn spawn_walls_in_locations(locations: Vec<GridLocation>, board: &mut TileTypeBoard){
    for location in locations{
        board.set(&location, Tile::new(TileType::Wall));
    }
}