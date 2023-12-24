use crate::{prelude::*, output::{error_handler, print_to_console}, costume_event::{board_set_event, move_tile_event}};

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(Update, (
                    move_tile_logic,
                    conduct_new_board_building
                )
                .chain()
                .in_set(InputSystemSets::InputHandling)
            )
            ;
    }
}

pub fn conduct_new_board_building(
    mut event_listener: EventReader<board_set_event::ConductANewBoardBuilding>,
    mut graphics_event_writer: EventWriter<board_set_event::MoveExistingTilesGraphics>,
    
    //put here event writer for board builder

    existing_tiles: Query<Entity, With<TileType>>
){
    for build_conduction_request in event_listener.read(){
        //make sure the board builder also gets notified 
        //whether a new solved board should be created



                // if existing_tiles.iter().len() == 
        //     power( board_size.to_grid_side_length() )
        // {
        // // // // example: graphics_event_writer.send(board_set_event::MoveExistingTilesGraphics); 
        //     //don't mess with spawns oe despawns
        // }else{
        //     //calculate what should be spawned or de and summon their events
        // }
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