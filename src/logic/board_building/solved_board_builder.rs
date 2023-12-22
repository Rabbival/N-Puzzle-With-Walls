use crate::prelude::*;

pub struct SolvedBoardBuilderPlugin;

impl Plugin for SolvedBoardBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, spawn_solved_board)
            ;
    }
}

fn spawn_solved_board(
    mut commands: Commands,
    board_prop_res: Res<BoardProperties>,
){
    commands.spawn((generate_solved_board(board_prop_res.size.to_grid_side_length()), SolvedBoard));
}

/// public for the sake of testing
pub fn generate_solved_board(grid_side_length: u8) -> TileTypeBoard{
    let mut solved_board = TileTypeBoard::new(grid_side_length);
    let grid_side_length_u32 = grid_side_length as u32;
    for i in 0..grid_side_length_u32{
        for j in 0..grid_side_length_u32{
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set(&location, TileType::new(Some(i*grid_side_length_u32+j+1)));
        }
    }
    let empty_tile_location=GridLocation::new((grid_side_length_u32-1) as i32, (grid_side_length_u32-1) as i32);
    solved_board.set(&empty_tile_location, TileType::new(None));
    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    solved_board
}