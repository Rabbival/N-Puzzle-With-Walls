use crate::prelude::*;

pub fn generate_solved_board(grid_side_length: u8) -> TileTypeBoard{
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;

    //Testing wall locations
    solved_board.set(
        &GridLocation{row: 1, col: 1}, 
        IndexedValue::<TileType>::new(TileType::Wall)
    );
    solved_board.set(
        &GridLocation{row: 1, col: 3}, 
        IndexedValue::<TileType>::new(TileType::Wall)
    );

    for i in 0..grid_side_length_u32{
        for j in 0..grid_side_length_u32{
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set_if_empty(&location, IndexedValue::<TileType>::new(TileType::Numbered));
        }
    }
    let empty_tile_location=GridLocation::new((grid_side_length_u32-1) as i32, (grid_side_length_u32-1) as i32);
    solved_board.set(&empty_tile_location, IndexedValue::<TileType>::new(TileType::Empty));

    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    solved_board
}