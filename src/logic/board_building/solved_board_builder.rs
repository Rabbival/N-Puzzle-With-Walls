use crate::prelude::*;

pub fn generate_solved_board(grid_side_length: u8) -> TileTypeBoard{
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;

    //Testing wall locations
    solved_board.set(&GridLocation{row: 1, col: 1}, TileType::Wall(0));
    solved_board.set(&GridLocation{row: 1, col: 3}, TileType::Wall(1));
    solved_board.set(&GridLocation{row: 3, col: 0}, TileType::Wall(2));
    solved_board.set(&GridLocation{row: 3, col: 1}, TileType::Wall(3));
    solved_board.set(&GridLocation{row: 3, col: 2}, TileType::Wall(4));
    solved_board.set(&GridLocation{row: 3, col: 3}, TileType::Wall(5));

    for i in 0..grid_side_length_u32{
        for j in 0..grid_side_length_u32{
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set_if_empty(&location, TileType::Numbered(0));
        }
    }
    let empty_tile_location=GridLocation::new((grid_side_length_u32-1) as i32, (grid_side_length_u32-1) as i32);
    solved_board.set(&empty_tile_location, TileType::Empty(0));

    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    solved_board
}