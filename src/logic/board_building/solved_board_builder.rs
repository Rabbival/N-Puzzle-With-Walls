use crate::prelude::*;
use rand::Rng;

const MIN_NEIGHBORS: u8 = 1;

pub fn generate_solved_board(applied_props: &BoardProperties) -> TileTypeBoard{
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;

    if applied_props.wall_count > 0 {
        spawn_walls_in_locations(
            determine_wall_locations(applied_props.wall_count, grid_side_length), 
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

    info!("{:?}", solved_board.grid.is_strongly_connected());

    solved_board.index_all_tile_types();
    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    solved_board
}

fn determine_wall_locations(wall_count: u8, grid_side_length: u8) -> Vec<GridLocation>{
    let mut rng = rand::thread_rng();
    let mut wall_spawn_locations = vec![];
    let mut neighbor_count_grid = initialize_neighbor_count_grid(grid_side_length);
    let mut possible_spawn_locations = neighbor_count_grid.all_locations_as_vec();

    // NTS: if MIN_NEIGHBORS will be changed to 2, should start without the corners' neighbors,
    //      if 3, we should also start without neighbors of edges

    for _ in 0..wall_count{
        if possible_spawn_locations.is_empty() {
            break;
        }
        let chosen_wall_location_index = rng.gen_range(0..possible_spawn_locations.len());
        let chosen_wall_location = possible_spawn_locations[chosen_wall_location_index];
        wall_spawn_locations.push(chosen_wall_location);
        //can't choose the same tile again
        remove_by_value(
            &chosen_wall_location, 
            &mut possible_spawn_locations
        );
        for neighbor in 
            neighbor_count_grid.get_all_direct_neighbor_locations(&chosen_wall_location)
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
    wall_spawn_locations
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