use crate::{prelude::*, output::{error_handler, print_to_console, graphics}};
use rand::Rng;

const LOCATION_SHIFT_BOUNDS:(u8, u8) = (18, 22);
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
            ;
    }
}

fn spawn_solved_board(mut commands: Commands){
    commands.spawn((generate_solved_board(), SolvedBoard));
}

fn generate_solved_board() -> Board{
    let mut solved_board = Board::default();
    for i in 0..GRID_SIZE as u32 {
        for j in 0..GRID_SIZE as u32 {
            let location = GridLocation::new(i as i32, j as i32);
            solved_board[&location] = Tile::new(Some(i*GRID_SIZE+j+1));
        }
    }
    let empty_tile_location=GridLocation::new((GRID_SIZE-1) as i32, (GRID_SIZE-1) as i32);
    solved_board[&empty_tile_location] = Tile::new(None);
    solved_board.empty_tile_location=empty_tile_location;
    solved_board.ignore_player_input=true;
    solved_board
}

fn spawn_game_board(mut commands: Commands, query: Query<&Board, With<SolvedBoard>>){
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
fn generate_game_board(mut board: Board) -> Result<Board, error_handler::BoardGenerationError>
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
            return Err(error_handler::BoardGenerationError::DirectionNotFoundInMap);
        }
        let chosen_location=chosen_location_option.unwrap();
        board.switch_tiles_by_location(&empty_tile_location, chosen_location);
        
        //get ready for next choice
        empty_tile_location=chosen_location.clone();
        shift_direction_sequence.push(*chosen_direction);
        previous_shift_direction=*chosen_direction;
    }

    //generation was successful
    let reveresed_shift_order=shift_direction_sequence
        .iter()
        .rev()
        .map(|direction| -> BasicDirection {
            direction.opposite_direction().unwrap()
        });
    print_to_console::print_possible_solution(reveresed_shift_order);
    board.ignore_player_input=false;
    board.empty_tile_location=empty_tile_location;
    Ok(board)
}

/// Had to make the query optional for the sake of testing,
/// for the sake of the game- it's always there
pub fn move_tile_logic(
    occupied_tile_location: GridLocation, 
    optional_tiles: Option<Query<&mut Transform, With<Tile>>>,
    game_board: &mut Board,
    solved_board: &Board,
) -> Result<(), error_handler::TileMoveError>
{
    if game_board.ignore_player_input{
        return Err(TileMoveError::BoardFrozenToPlayer(String::from("board locked")));
    }
    if !game_board.occupied(&occupied_tile_location) {
        return Err(TileMoveError::PressedEmptySlot(String::from("pressed an empty slot")));
    }
    let optional_empty_neighbor_location= 
        game_board.get_empty_neighbor(&occupied_tile_location);
    if let None=optional_empty_neighbor_location{
        return Err(TileMoveError::NoEmptyNeighbor(String::from("no empty neighbor")));
    }
    let empty_neighbor_location=optional_empty_neighbor_location.unwrap();
    
    if let Some(tiles) = optional_tiles{
        graphics::switch_tile_entity_positions(
            tiles, 
            &game_board,
            &occupied_tile_location, 
            &empty_neighbor_location
        )?;
        game_log(GameLog::TilesMoved(empty_neighbor_location))
    }

    // it's important that we do it after moving the entities so that graphic transition code is more readable
    game_board.switch_tiles_by_location(&empty_neighbor_location, &occupied_tile_location);

    check_if_solved(game_board, solved_board);

    return Ok(());
}

fn check_if_solved(game_board: &mut Board, solved_board: &Board){
    if game_board == solved_board {
        game_log(GameLog::Victory);
        game_board.ignore_player_input=true;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_tile_logic() {
        assert!(test_locked_board());
        assert!(test_empty_slot());
        assert!(test_no_empty_neighbor());
    }

    #[test]
    fn several_attempts_at_generating_unsolved_boards(){
        const ATTEMPT_COUNT: u8 = 10;
        let solved_board=generate_solved_board();
        for _ in 0..ATTEMPT_COUNT{
            assert_ne!(solved_board, 
                match generate_game_board(solved_board.clone()){
                    Ok(board)=> board,
                    Err(_)=> panic!()
                }
            );
        }
    }

    fn test_locked_board()-> bool{
        let location_search_outcome=
            move_tile_logic(
                GridLocation::default(), 
                None,
                &mut Board::default(), //locked be default
                &Board::default()
            );
        match location_search_outcome{
                Err(TileMoveError::BoardFrozenToPlayer(_))=> true,
                _ => false
            }
    }

    fn test_empty_slot()-> bool{
        let mut board=Board::default();
        board.ignore_player_input=false;
        let location_search_outcome=
            move_tile_logic(
                GridLocation::default(), 
                None,
                &mut board,
                &Board::default()
            );
        match location_search_outcome{
                Err(TileMoveError::PressedEmptySlot(_))=> true,
                _ => false
            }
    }

    fn test_no_empty_neighbor()-> bool{
        let mut board=generate_solved_board();
        board.ignore_player_input=false;
        let empty_tile_location=board.empty_tile_location;
        board[&empty_tile_location]=Tile::new(Some(16));
        let location_search_outcome=
            move_tile_logic(
                GridLocation { row: 0, col: 0 }, 
                None,
                &mut board,
                &Board::default()
            );
        match location_search_outcome{
                Err(TileMoveError::NoEmptyNeighbor(_))=> true,
                _ => false
            }
    }
}