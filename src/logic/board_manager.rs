use crate::{prelude::*, output::{error_handler, print_to_console}, costume_event::move_tile_event};

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app    
            .add_systems(Update, (
                    move_tile_logic,
                    
                )
                .chain()
                .in_set(InputSystemSets::InputHandling)
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
        if let Err(move_error)=move_tile_logic_inner(
            &mut graphics_event_writer, 
            switch_tile_request.move_neighbor_from_direction, 
            switch_tile_request.empty_tile_index, 
            &mut game_board_query.single_mut(), 
            &solved_board_query.single().grid
        ){
            print_to_console::print_tile_move_error(move_error);
        }
    }
}

/// graphics switched before logic for the sake of graphics function readability
pub fn move_tile_logic_inner(
    graphics_event_writer: &mut EventWriter<move_tile_event::SwitchTilesGraphics>,
    move_neighbor_from_direction: BasicDirection, 
    empty_tile_index: usize, 
    game_board: &mut TileTypeBoard,
    solved_grid: &Grid<Tile>,
) -> Result<(), error_handler::TileMoveError>
{    
    if game_board.ignore_player_input{
        return Err(error_handler::TileMoveError::BoardFrozenToPlayer(String::from("board locked")));
    }

    let empty_tile_neighbors
        = game_board.get_direct_neighbors_of_empty(empty_tile_index);
    if let Some(&occupied_tile_location) 
        = empty_tile_neighbors.get(&move_neighbor_from_direction)
    {
        let optional_occupied_tile = game_board.get(&occupied_tile_location);
        if optional_occupied_tile.is_none() {
            return Err(error_handler::TileMoveError::NoTileInCell(occupied_tile_location));
        } else if optional_occupied_tile.unwrap().tile_type == TileType::Wall {
            return Err(error_handler::TileMoveError::TriedToSwitchWithAWall);
        }
        let empty_tile_location = game_board
            .get_empty_tile_location_by_index(empty_tile_index);

        game_board.swap_tiles_by_location(&empty_tile_location, &occupied_tile_location)?;

        // reminder that from this point the logic locations are swapped
    
        graphics_event_writer.send(move_tile_event::SwitchTilesGraphics{
            first_grid_location: occupied_tile_location,
            second_grid_location: empty_tile_location
        });
    
        print_to_console::game_log(GameLog::TilesMoved(
            game_board.get(&empty_tile_location).unwrap(),
            &empty_tile_location
        ));
    
        check_if_solved(game_board, solved_grid);
        Ok(())
    }else{
        Err(error_handler::TileMoveError
                ::NoOccupiedTileInThatDirection(move_neighbor_from_direction))
    }
}

/// also freezes the board if it is solved
fn check_if_solved(game_board: &mut TileTypeBoard, solved_grid: &Grid<Tile>){
    if game_board.grid == *solved_grid {
        print_to_console::game_log(GameLog::Victory);
        game_board.ignore_player_input=true;
    }
}


#[cfg(test)]
mod tests {
    use crate::logic::{board_building::solved_board_builder, enums::basic_direction};

    use super::*;

    #[test]
    fn test_valid_request(){
        let mut app = App::new();
        app
            .add_event::<move_tile_event::SwitchTilesGraphics>()
            .add_systems(Update, test_valid_request_inner)
        ;
        app.update();
    }

    fn test_valid_request_inner(mut event_writer: EventWriter::<move_tile_event::SwitchTilesGraphics>) {
        assert!( ! detected_as_invalid_request(
            basic_direction::BasicDirection::Up,
            &mut event_writer
        ));
        assert!(detected_as_invalid_request(
            basic_direction::BasicDirection::Right,
            &mut event_writer
        ));
        assert!(detected_as_invalid_request(
            basic_direction::BasicDirection::Down,
            &mut event_writer
        ));
        assert!( ! detected_as_invalid_request(
            basic_direction::BasicDirection::Left,
            &mut event_writer
        ));
    }

    fn detected_as_invalid_request(
        from_dir: basic_direction::BasicDirection,
        event_writer: &mut EventWriter::<move_tile_event::SwitchTilesGraphics>
    )-> bool
    {
        let mut board
            =solved_board_builder::generate_solved_board(&BoardProperties::default()).unwrap();
        board.ignore_player_input=false;
        let direction_check_outcome=
            move_tile_logic_inner(
                event_writer,
                from_dir, 
                0,
                &mut board.clone(),
                &board.grid
            );
        match direction_check_outcome{
            Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(_))=> true,
            _=> false
        }
    }
}