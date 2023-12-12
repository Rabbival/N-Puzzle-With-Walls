use crate::{prelude::*, logic::{board_manager, basic_direction, tile_dictionary}, output::{print_to_console, error_handler}};

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (listen_for_reset, move_tiles_with_keyboard));
    }
}

fn move_tiles_with_keyboard(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_board_query: Query<&mut TileTypeBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&TileTypeBoard, (With<SolvedBoard>, Without<GameBoard>)>,
    tiles: Query<&mut Transform, With<TileType>>
){
    let mut move_request_direction:Option<basic_direction::BasicDirection>=None;
    if keyboard_input.just_pressed(KeyCode::W) ||  keyboard_input.just_pressed(KeyCode::Up){
        move_request_direction=Some(basic_direction::BasicDirection::Down);
    }
    if keyboard_input.just_pressed(KeyCode::D) ||  keyboard_input.just_pressed(KeyCode::Right){
        move_request_direction=Some(basic_direction::BasicDirection::Left);
    }
    if keyboard_input.just_pressed(KeyCode::S) ||  keyboard_input.just_pressed(KeyCode::Down){
        move_request_direction=Some(basic_direction::BasicDirection::Up);
    }
    if keyboard_input.just_pressed(KeyCode::A) ||  keyboard_input.just_pressed(KeyCode::Left){
        move_request_direction=Some(basic_direction::BasicDirection::Right);
    }
    if let None = move_request_direction {
        return;
    }
    if let Err(error) = move_into_empty_from_direction(
        move_request_direction.unwrap(),
        game_board_query.single_mut().into_inner(),
        &solved_board_query.single().grid,
        Some(tiles)
    ){
        print_to_console::print_input_error(error)
    }
}

fn move_into_empty_from_direction(
    move_to_direction: basic_direction::BasicDirection,
    game_board: &mut TileTypeBoard,
    solved_grid: &Grid<TileType>,
    optional_tiles: Option<Query<&mut Transform, With<TileType>>>
) -> Result<(), error_handler::TileMoveError>
{
    if game_board.ignore_player_input{
        return Err(error_handler::TileMoveError::BoardFrozenToPlayer(String::from("board locked")));
    }
    let empty_tile_neighbors=game_board.get_direct_neighbors_of_empty();
    if let Some(occupied_tile_location) = empty_tile_neighbors.get(&move_to_direction){
        if let Some(tiles) = optional_tiles{
            return board_manager::move_tile_logic(
                *occupied_tile_location, 
                game_board.empty_tile_location,
                game_board, 
                solved_grid,
                tiles
            )
        }
        Ok(()) //only here for the sake of testing, there will always be tiles.
    }else{
        Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(move_to_direction))
    }
}

fn listen_for_reset(
    solved_board_query: Query<&TileTypeBoard,(With<SolvedBoard>, Without<GameBoard>)>,
    mut game_board_query: Query<&mut TileTypeBoard,(With<GameBoard>, Without<SolvedBoard>)>,
    tiles: Query<(Entity, &mut TileType, &mut Transform)>,
    mut tile_dictionary_query: Query<&mut tile_dictionary::TileDictionary, With<tile_dictionary::TileDictionaryTag>>,
    keyboard_input: Res<Input<KeyCode>>
){
    if keyboard_input.just_pressed(KeyCode::R){
        if let Err(error) = 
            board_manager::reset_board(
                &solved_board_query.single().grid,
                &mut game_board_query.single_mut(),
                tiles,
                &mut tile_dictionary_query.single_mut().entity_by_tile_type
            )
        {
            print_to_console::print_debug_deriver(error, BevyPrintType::Error);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_request() {
        assert!( ! detected_as_invalid_request(basic_direction::BasicDirection::Up));
        assert!(detected_as_invalid_request(basic_direction::BasicDirection::Right));
        assert!(detected_as_invalid_request(basic_direction::BasicDirection::Down));
        assert!( ! detected_as_invalid_request(basic_direction::BasicDirection::Left));
    }

    fn detected_as_invalid_request(from_dir: basic_direction::BasicDirection)-> bool{
        let mut board=board_manager::generate_solved_board();
        board.ignore_player_input=false;
        let direction_check_outcome=
            move_into_empty_from_direction(
                from_dir, 
                &mut board,
                &TileTypeBoard::default().grid,
                None
            );

        println!("for {:?}, {:?}", from_dir, direction_check_outcome);

        match direction_check_outcome{
            Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(_))=> true,
            _=> false
        }
    }
}