use crate::prelude::*;

pub struct SolvedBoardBuilderPlugin;

impl Plugin for SolvedBoardBuilderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreStartup, spawn_solved_board)
            ;
    }
}


fn spawn_solved_board(mut commands: Commands){
    commands.spawn((generate_solved_board(), SolvedBoard));
}

/// public for the sake of testing
pub fn generate_solved_board() -> TileTypeBoard{
    let mut solved_board = TileTypeBoard::default();
    let grid_side_length = solved_board.get_side_length().clone() as u32;
    for i in 0..grid_side_length as u32 {
        for j in 0..grid_side_length as u32 {
            let location = GridLocation::new(i as i32, j as i32);
            solved_board.set(&location, TileType::new(Some(i*grid_side_length+j+1)));
        }
    }
    let empty_tile_location=GridLocation::new((grid_side_length-1) as i32, (grid_side_length-1) as i32);
    solved_board.set(&empty_tile_location, TileType::new(None));
    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    solved_board
}