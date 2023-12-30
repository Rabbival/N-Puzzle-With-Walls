use crate::prelude::*;

pub fn generate_solved_board(applied_props: &BoardProperties) -> TileTypeBoard{
    let grid_side_length = applied_props.size.to_grid_side_length();
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;

    //Testing wall locations
    let test_vec = vec![
        GridLocation{row: 1, col: 1},
        GridLocation{row: 1, col: 3},
        GridLocation{row: 3, col: 0},
        GridLocation{row: 3, col: 1},
        GridLocation{row: 3, col: 2},
        GridLocation{row: 3, col: 3},
    ];
    spawn_walls_in_locations(test_vec , &mut solved_board);

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
    solved_board
}

fn determine_wall_locations(grid_side_length: u8){
    let mut neighbor_count_grid = initialize_neighbor_count_grid(grid_side_length);

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