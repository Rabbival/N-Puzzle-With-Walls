use crate::{prelude::*, output::{error_handler, print_to_console}, costume_event::{reset_event, move_tile_event}};
use rand::Rng;

const LOCATION_SHIFT_BOUNDS:(u8, u8) = (18, 25);
const BOARD_GENERATION_ATTEMPTS:u8=5;

#[derive(Component)]
pub struct SolvedBoard;
#[derive(Component)]
pub struct GameBoard;

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            //important to run before we draw it in graphics.rs
            .add_systems(PreStartup, spawn_solved_board)
            .add_systems(Startup, spawn_game_board)
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

fn spawn_game_board(mut commands: Commands, query: Query<&TileTypeBoard, With<SolvedBoard>>){
    let solved_board=query.single();
    for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
        let attempt_result=generate_game_board(solved_board.clone());
         //generation successful
        if let Ok(board) = attempt_result { 
            commands.spawn((
                board,
                GameBoard
            ));
            return; 
        }
    }
    print_to_console::couldnt_generate_board();
}

/// a permutation that was made from shifts in a solved board 
/// would always be solvable (if we shift in reverse)
fn generate_game_board(mut board: TileTypeBoard) -> Result<TileTypeBoard, error_handler::BoardGenerationError>
{
    let mut rng = rand::thread_rng();
    let mut location_shift_count=rng.gen_range(LOCATION_SHIFT_BOUNDS.0..LOCATION_SHIFT_BOUNDS.1);
    //prevents the generation of another solved board
    if location_shift_count%2 == 0 {
        location_shift_count+=1;
    }
    let mut empty_tile_location=board.empty_tile_location;

    let mut shift_direction_sequence:Vec<BasicDirection> = vec!();
    //we'll never shift with the location below on the first shift since there's none
    let mut previous_shift_direction = BasicDirection::Up; 
    for _shift in 0..location_shift_count{
        let mut optional_directions=
            board.get_all_direct_neighbor_locations(&empty_tile_location);

        //don't want to shift back and forth
        let opposite_of_previous_shift=previous_shift_direction.opposite_direction();
        if let None = opposite_of_previous_shift{
            return Err(error_handler::BoardGenerationError::DirectionCouldntBeFlipped);
        }
        optional_directions.remove(&opposite_of_previous_shift.unwrap());

        //choose, register, update board
        let valid_directions:Vec<&BasicDirection>=optional_directions.keys().clone().collect(); 
        let chosen_shift_index=rng.gen_range(0..optional_directions.len());
        let chosen_direction=valid_directions[chosen_shift_index];
        let chosen_location_option=optional_directions.get(chosen_direction);
        if let None = chosen_location_option{
            return Err(error_handler::BoardGenerationError::ItemNotInMap
                (ItemNotFoundInMapError::DirectionNotFoundInMap));
        }
        let chosen_location=chosen_location_option.unwrap();
        if let Err(_) = 
            board.switch_tiles_by_location(&empty_tile_location, chosen_location){
                return Err(error_handler::BoardGenerationError::TileMoveError);
            }
        
        //get ready for next choice
        empty_tile_location=board.empty_tile_location;
        shift_direction_sequence.push(*chosen_direction);
        previous_shift_direction=*chosen_direction;
    }

    //generation was successful
    let reveresed_shift_order=shift_direction_sequence
        .iter()
        .rev()
        .map(|direction| -> BasicDirection {
            *direction
        });
    print_to_console::print_possible_solution(reveresed_shift_order);
    board.ignore_player_input=false;
    Ok(board)
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
        first_grid_location: occupied_tile_location.clone(),
        second_grid_location: empty_tile_location.clone()
    });
    if game_board.get(&occupied_tile_location) == None {
        return Err(error_handler::TileMoveError::NoTileInCell(occupied_tile_location));
    }
    print_to_console::game_log(GameLog::TilesMoved(
        game_board.get(&occupied_tile_location).unwrap(),
        &empty_tile_location
    ));

    game_board.switch_tiles_by_location(&empty_tile_location, &occupied_tile_location)?;

    check_if_solved(game_board, solved_grid);

    return Ok(());
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
    solved_board_query: Query<&TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
){
    for _reset_request in reset_listener.read(){
        let solved_grid=&solved_board_query.single().grid;
        let mut game_board=game_board_query.single_mut();
        for _attempt in 0..BOARD_GENERATION_ATTEMPTS{
            let attempt_result=
                generate_game_board(TileTypeBoard::from_grid(solved_grid));
             //generation successful
            if let Ok(board) = attempt_result { 
                *game_board=board;
                graphics_event_writer.send(reset_event::ResetBoardGraphics::default());
                return;
            }
        }
        print_to_console::couldnt_generate_board();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn several_attempts_at_generating_unsolved_boards(){
        const ATTEMPT_COUNT: u8 = 10;
        let solved_board=generate_solved_board();
        for _ in 0..ATTEMPT_COUNT{
            assert_ne!(solved_board.grid, 
                match generate_game_board(solved_board.clone()){
                    Ok(board)=> board,
                    Err(_)=> panic!()
                }.grid
            );
        }
    }
}