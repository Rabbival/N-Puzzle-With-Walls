use crate::{prelude::*, output::error_handler};
use bevy::utils::hashbrown::HashMap;
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
    // TODO: update the following when I enable the option for more
    let empty_tile_location=GridLocation::new((grid_side_length_u32-1) as i32, (grid_side_length_u32-1) as i32);

    for i in 0..grid_side_length_u32{
        for j in 0..grid_side_length_u32{
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set_if_none(&location, Tile::new(TileType::Numbered));
        }
    }

    solved_board.index_all_tile_types();
    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    Ok(solved_board)
}

fn determine_wall_locations(wall_count: u8, grid_side_length: u8) 
-> Result<Vec<GridLocation>, error_handler::BoardGenerationError>
{
    let mut rng = rand::thread_rng();
    let mut wall_spawn_locations = vec![];
    let mut neighbor_count_grid = initialize_neighbor_count_grid(grid_side_length);
    let mut possible_spawn_locations = neighbor_count_grid.all_locations_as_vec();

    for _ in 0..wall_count{
        let mut chosen_wall_location = GridLocation::default();
        let mut neighbors_of_chosen_wall_location 
            = HashMap::<BasicDirection,GridLocation>::new() ;
        let mut valid_spawn_location = false;
        while ! possible_spawn_locations.is_empty() && !valid_spawn_location{
            let chosen_wall_location_index = rng.gen_range(0..possible_spawn_locations.len());
            chosen_wall_location = possible_spawn_locations[chosen_wall_location_index];
    
            //check if removing that tile keeps the graph connected, 
            //if not - put it back, and reroll
            let chosen_tile_value 
                = neighbor_count_grid.set_none_get_former(&chosen_wall_location);
            valid_spawn_location = true;
            if neighbor_count_grid.is_connected_graph(){
                // check whether choosing that location brings a tile bellow the minimal neighbor counts
                neighbors_of_chosen_wall_location 
                    = neighbor_count_grid.get_all_direct_neighbor_locations(&chosen_wall_location);
                for neighbor_of_chosen in neighbors_of_chosen_wall_location.values(){
                    if *neighbor_count_grid.get(neighbor_of_chosen).unwrap() == MIN_NEIGHBORS {
                        valid_spawn_location = false;
                        break;
                    }
                }
            }else{
                neighbor_count_grid.set(&chosen_wall_location, chosen_tile_value.unwrap());
                valid_spawn_location = false;
            }

            // whether it's because the chosen location is illegal 
            // or because we don't want to choose the same location twice
            // the chosen location has to be removed from the available ones
            remove_by_value(
                &chosen_wall_location, 
                &mut possible_spawn_locations
            );
        }
        if possible_spawn_locations.is_empty() {
            return Err(error_handler::BoardGenerationError::CouldntPlaceAllWalls);
        }

        // if the location was chosen
        wall_spawn_locations.push(chosen_wall_location);       
        for neighbor in neighbors_of_chosen_wall_location
        {
            let neighbor_location = neighbor.1;
            let neighbor_value = neighbor_count_grid.get_mut(&neighbor_location).unwrap();
            *neighbor_value -= 1;

            // if a neigbor of the chosen location got to the threshold
            if *neighbor_value == MIN_NEIGHBORS{
                // we can't put a wall in its neighbor or it'll go below threshold
                for neighbor_to_forbid in 
                    neighbor_count_grid.get_all_direct_neighbor_locations(&neighbor_location)
                {
                    remove_by_value(
                        &neighbor_to_forbid.1, 
                        &mut possible_spawn_locations
                    );
                }
            }
        }
    }
    Ok(wall_spawn_locations)
}

fn remove_by_value(location_to_forbid: &GridLocation, possible_spawn_locations: &mut Vec<GridLocation>){
    let optional_index_to_remove = possible_spawn_locations.iter()
        .position(|x| *x == *location_to_forbid);
    //could be that it was removed before by a different neighbor
    if let Some(index_to_remove) =  optional_index_to_remove {
        possible_spawn_locations.swap_remove(index_to_remove);
    }
}

fn initialize_neighbor_count_grid(grid_side_length: u8) -> Grid<u8>{
    let mut grid = Grid::new(grid_side_length);
    for corner in grid.corner_locations(){
        grid.set(&corner, 2);
    }
    for edge_tile_not_corner in grid.edges_without_corners_locations(){
        grid.set(&edge_tile_not_corner, 3);
    }
    for inner_cell in grid.all_locations_no_edges(){
        grid.set(&inner_cell, 4);
    }
    grid
}

fn spawn_walls_in_locations(locations: Vec<GridLocation>, board: &mut TileTypeBoard){
    for location in locations{
        board.set(&location, Tile::new(TileType::Wall));
    }
}