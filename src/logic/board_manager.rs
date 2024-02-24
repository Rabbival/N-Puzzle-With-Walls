use crate::{
    costume_event::move_tile_event,
    output::{error_handler, print_to_console},
    prelude::*,
};

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                    move_tile_logic,
                    check_if_solved
                )
                .chain()
                .in_set(InputSystemSets::InputHandling),
        );
    }
}

/// graphics switched before logic for the sake of graphics function readability
fn move_tile_logic(
    mut graphics_event_writer: EventWriter<move_tile_event::UpdateTileLocationGraphics>,
    mut check_if_board_is_solved_writer: EventWriter<move_tile_event::CheckIfBoardIsSolved>,
    mut logic_event_reader: EventReader<move_tile_event::SwitchTilesLogic>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
) {
    for switch_tile_request in logic_event_reader.read() {
        if let Err(move_error) = move_tile_logic_inner(
            &mut graphics_event_writer,
            &mut check_if_board_is_solved_writer,
            switch_tile_request.move_neighbor_from_direction,
            switch_tile_request.empty_tile_index,
            &mut game_board_query.single_mut(),
        ) {
            print_to_console::print_tile_move_error(move_error);
        }
    }
}

/// graphics switched before logic for the sake of graphics function readability
fn move_tile_logic_inner(
    graphics_event_writer: &mut EventWriter<move_tile_event::UpdateTileLocationGraphics>,
    check_if_board_is_solved_writer: &mut EventWriter<move_tile_event::CheckIfBoardIsSolved>,
    move_neighbor_from_direction: BasicDirection,
    empty_tile_index: usize,
    game_board: &mut TileBoard,
) -> Result<(), error_handler::TileMoveError> {
    if game_board.ignore_player_input {
        return Err(error_handler::TileMoveError::BoardFrozenToPlayer);
    }

    let empty_tile_neighbors = game_board.get_direct_neighbors_of_empty(empty_tile_index);
    if let Some(&occupied_tile_location) = empty_tile_neighbors.get(&move_neighbor_from_direction) {
        let optional_occupied_tile = 
            wrap_if_error(game_board.get(&occupied_tile_location))?;
        if optional_occupied_tile.is_none() {
            return Err(error_handler::TileMoveError::TileBoardError
                (TileBoardError::NoTileInCell(occupied_tile_location)));
        }
        let occupied_tile = *optional_occupied_tile.unwrap();
        if occupied_tile.tile_type == TileType::Wall {
            return Err(error_handler::TileMoveError::TriedToSwitchWithAWall);
        }

        let empty_tile_location = *game_board.get_empty_tile_location(empty_tile_index);
        game_board.swap_tiles_by_location(&empty_tile_location, &occupied_tile_location)?;

        // reminder that from this point the logic locations are swapped

        print_to_console::game_log(GameLog::TilesMoved(&occupied_tile, &empty_tile_location));

        graphics_event_writer.send(move_tile_event::UpdateTileLocationGraphics {
            tile: occupied_tile,
            new_location: empty_tile_location,
        });
        graphics_event_writer.send(move_tile_event::UpdateTileLocationGraphics {
            tile: *wrap_if_error(game_board.get_empty_tile(empty_tile_index))?.unwrap(),
            new_location: occupied_tile_location,
        });

        check_if_board_is_solved_writer.send(move_tile_event::CheckIfBoardIsSolved);

        Ok(())
    } else {
        Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(
            move_neighbor_from_direction,
        ))
    }
}

/// also freezes the board if it is solved
fn check_if_solved(
    mut check_if_board_is_solved_listener: EventReader<move_tile_event::CheckIfBoardIsSolved>,
    mut set_game_state_to_victory: ResMut<NextState<GameState>>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&TileBoard, (With<SolvedBoard>, Without<GameBoard>)>,
) {
    for _check_request in check_if_board_is_solved_listener.read(){
        if game_board_query.single().grid == solved_board_query.single().grid {
            set_game_state_to_victory.set(GameState::Victory);
            print_to_console::game_log(GameLog::Victory);
            game_board_query.single_mut().ignore_player_input = true;
        }
    }
}

/// I don't use it automatically inside the get set etc functions
/// since it they might have nothing to do with moving tiles
fn wrap_if_error<T>(result: Result<T, error_handler::GridError>) 
-> Result<T, error_handler::TileMoveError>{
    match result {
        Err(grid_error) => {
            Err(error_handler::TileMoveError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use crate::costume_event::db_event;
    use crate::logic::{board_building::solved_board_builder, enums::basic_direction};

    use super::*;

    #[test]
    fn test_valid_request() {
        let mut app = App::new();
        app.add_event::<move_tile_event::UpdateTileLocationGraphics>()
            .add_event::<move_tile_event::CheckIfBoardIsSolved>()
            .add_event::<db_event::SaveToDB>()
            .add_systems(Update, test_valid_request_inner);
        app.update();
    }

    fn test_valid_request_inner(
        mut graphics_writer: EventWriter<move_tile_event::UpdateTileLocationGraphics>,
        mut check_writer: EventWriter<move_tile_event::CheckIfBoardIsSolved>,
        mut db_writer: EventWriter<db_event::SaveToDB>,
    ) {
        assert!(!detected_as_invalid_request_inner(
            basic_direction::BasicDirection::Up,
            &mut graphics_writer,
            &mut check_writer,
            &mut db_writer
        ));
        assert!(detected_as_invalid_request_inner(
            basic_direction::BasicDirection::Right,
            &mut graphics_writer,
            &mut check_writer,
            &mut db_writer
        ));
        assert!(detected_as_invalid_request_inner(
            basic_direction::BasicDirection::Down,
            &mut graphics_writer,
            &mut check_writer,
            &mut db_writer
        ));
        assert!(!detected_as_invalid_request_inner(
            basic_direction::BasicDirection::Left,
            &mut graphics_writer,
            &mut check_writer,
            &mut db_writer
        ));
    }

    fn detected_as_invalid_request_inner(
        from_dir: basic_direction::BasicDirection,
        graphics_writer: &mut EventWriter<move_tile_event::UpdateTileLocationGraphics>,
        check_writer: &mut EventWriter<move_tile_event::CheckIfBoardIsSolved>,
        db_writer: &mut EventWriter<db_event::SaveToDB>,
    ) -> bool {
        let mut board =
            solved_board_builder::generate_solved_board_inner(
                &BoardProperties::default(),
                db_writer
            ).unwrap();
        board.ignore_player_input = false;
        let direction_check_outcome =
            move_tile_logic_inner(graphics_writer, check_writer, from_dir, 0, &mut board.clone());
        match direction_check_outcome {
            Err(error_handler::TileMoveError::NoOccupiedTileInThatDirection(_)) => true,
            _ => false,
        }
    }
}
