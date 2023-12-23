use crate::{prelude::*, output::{error_handler, print_to_console}, costume_event::{reset_event, move_tile_event}};

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(Update, (
                    move_tile_logic,
                    reset_board
                )
                .chain()
                .in_set(CostumeSystemSets::InputHandling)
            )
            ;
    }
}


/// graphics switched before logic for the sake of graphics function readability
pub fn move_tile_logic(
    mut graphics_event_writer: EventWriter<move_tile_event::SwitchTilesGraphics>,
    mut logic_event_reader: EventReader<move_tile_event::SwitchTilesLogic>,
    mut game_board_query: Query<&mut TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
){    
    for switch_tile_request in logic_event_reader.read(){
        if let Err(move_error)=inner_move_tile_logic(
            &mut graphics_event_writer, 
            switch_tile_request.occupied_tile_location, 
            switch_tile_request.empty_tile_location, 
            &mut game_board_query.single_mut(), 
            &solved_board_query.single().grid
        ){
            print_to_console::print_tile_move_error(move_error);
        }
    }
}

/// graphics switched before logic for the sake of graphics function readability
pub fn inner_move_tile_logic(
    graphics_event_writer: &mut EventWriter<move_tile_event::SwitchTilesGraphics>,
    occupied_tile_location: GridLocation, 
    empty_tile_location: GridLocation, 
    game_board: &mut TileTypeBoard,
    solved_grid: &Grid<TileType>,
) -> Result<(), error_handler::TileMoveError>
{    
    graphics_event_writer.send(move_tile_event::SwitchTilesGraphics{
        first_grid_location: occupied_tile_location,
        second_grid_location: empty_tile_location
    });
    if game_board.get(&occupied_tile_location).is_none() {
        return Err(error_handler::TileMoveError::NoTileInCell(occupied_tile_location));
    }
    print_to_console::game_log(GameLog::TilesMoved(
        game_board.get(&occupied_tile_location).unwrap(),
        &empty_tile_location
    ));

    game_board.switch_tiles_by_location(&empty_tile_location, &occupied_tile_location)?;

    check_if_solved(game_board, solved_grid);

    Ok(())
}

/// also freezes the board if it is solved
fn check_if_solved(game_board: &mut TileTypeBoard, solved_grid: &Grid<TileType>){
    if game_board.grid == *solved_grid {
        print_to_console::game_log(GameLog::Victory);
        game_board.ignore_player_input=true;
    }
}


pub fn reset_board(
    mut reset_listener: EventReader<reset_event::ResetBoardLogic>,
    mut graphics_event_writer: EventWriter<reset_event::ResetBoardGraphics>,
    mut solved_board_query: Query<&mut TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
    applied_board_prop_query: Query<
        &BoardProperties, 
        (With<AppliedBoardProperties>, Without<PlannedBoardProperties>)
    >,
){
    for reset_request in reset_listener.read(){
        let mut solved_board = solved_board_query.single_mut();
        let size = applied_board_prop_query.single().size;
        if reset_request.reroll_solved {
            *solved_board = generate_solved_board(size.to_grid_side_length());
        }
        let solved_grid = &solved_board.grid;
        let mut game_board=game_board_query.single_mut();
        for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
            let attempt_result=
                generate_game_board(
                    TileTypeBoard::from_grid(solved_grid), 
                    size.to_random_turns_range()
                );
             //generation successful
            if let Ok(board) = attempt_result { 
                *game_board=board;
                graphics_event_writer.send(reset_event::ResetBoardGraphics);
                return;
            }
        }
        print_to_console::couldnt_generate_board();
    }
}