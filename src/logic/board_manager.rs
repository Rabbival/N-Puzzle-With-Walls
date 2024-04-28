use crate::prelude::*;

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
    mut graphics_event_writer: EventWriter<UpdateTileLocationGraphics>,
    mut check_if_board_is_solved_writer: EventWriter<CheckIfBoardIsSolved>,
    mut logic_event_reader: EventReader<SwitchTilesLogic>,
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
            print_tile_move_error(move_error);
        }
    }
}

/// graphics switched before logic for the sake of graphics function readability
fn move_tile_logic_inner(
    graphics_event_writer: &mut EventWriter<UpdateTileLocationGraphics>,
    check_if_board_is_solved_writer: &mut EventWriter<CheckIfBoardIsSolved>,
    move_neighbor_from_direction: BasicDirection,
    empty_tile_index: usize,
    game_board: &mut TileBoard,
) -> Result<(), TileMoveError> {
    if game_board.ignore_player_input {
        return Err(TileMoveError::BoardFrozenToPlayer);
    }
    
    
    // info!("{:?}", game_board);
    

    let empty_tile_neighbors = game_board.get_direct_neighbors_of_empty(empty_tile_index);
    if let Some(&occupied_tile_original_location) = 
        empty_tile_neighbors.get(&move_neighbor_from_direction) 
    {
        let optional_occupied_tile = 
            wrap_if_error(game_board.get(&occupied_tile_original_location))?;
        if optional_occupied_tile.is_none() {
            return Err(TileMoveError::TileBoardError
                (TileBoardError::NoTileInCell(occupied_tile_original_location)));
        }
        let occupied_tile = *optional_occupied_tile.unwrap();
        if occupied_tile.tile_type == TileType::Wall {
            return Err(TileMoveError::TriedToSwitchWithAWall);
        }

        let empty_tile_original_location = *game_board.get_empty_tile_location(empty_tile_index);
        let empty_tile =
            *wrap_if_error(game_board.get_empty_tile(empty_tile_index))?.unwrap();
        game_board.swap_tiles_by_location(&empty_tile_original_location, &occupied_tile_original_location)?;

        // reminder that from this point the logic locations are swapped

        game_log(GameLog::TilesMoved(&occupied_tile, &empty_tile_original_location));

        graphics_event_writer.send(UpdateTileLocationGraphics {
            tile: occupied_tile,
            new_location: empty_tile_original_location,
        });
        graphics_event_writer.send(UpdateTileLocationGraphics {
            tile: empty_tile,
            new_location: occupied_tile_original_location,
        });

        check_if_board_is_solved_writer.send(CheckIfBoardIsSolved);

        Ok(())
    } else {
        Err(TileMoveError::NoOccupiedTileInThatDirection(
            move_neighbor_from_direction,
        ))
    }
}

/// also freezes the board if it is solved
fn check_if_solved(
    mut check_if_board_is_solved_listener: EventReader<CheckIfBoardIsSolved>,
    mut set_game_state_to_victory: ResMut<NextState<GameState>>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&TileBoard, (With<SolvedBoard>, Without<GameBoard>)>,
) {
    for _check_request in check_if_board_is_solved_listener.read(){
        if game_board_query.single().grid == solved_board_query.single().grid {
            set_game_state_to_victory.set(GameState::Victory);
            game_log(GameLog::Victory);
            game_board_query.single_mut().ignore_player_input = true;
        }
    }
}

/// I don't use it automatically inside the get set etc functions
/// since they might have nothing to do with moving tiles
fn wrap_if_error<T>(result: Result<T, GridError>)
-> Result<T, TileMoveError>{
    match result {
        Err(grid_error) => {
            Err(TileMoveError::GridError(grid_error))
        },
        Ok(value) => Ok(value)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_request() {
        let mut app = App::new();
        app
            .init_resource::<CurrentBoardWallLocations>()
            .add_event::<UpdateTileLocationGraphics>()
            .add_event::<CheckIfBoardIsSolved>()
            .add_systems(Update, test_valid_request_inner);
        app.update();
    }

    fn test_valid_request_inner(
        mut graphics_writer: EventWriter<UpdateTileLocationGraphics>,
        mut check_writer: EventWriter<CheckIfBoardIsSolved>,
        mut current_board_wall_locations: ResMut<CurrentBoardWallLocations>
    ) {
        assert!(!detected_as_invalid_request_inner(
            BasicDirection::Up,
            &mut graphics_writer,
            &mut check_writer,
            &mut current_board_wall_locations
        ));
        assert!(detected_as_invalid_request_inner(
            BasicDirection::Right,
            &mut graphics_writer,
            &mut check_writer,
            &mut current_board_wall_locations
        ));
        assert!(detected_as_invalid_request_inner(
            BasicDirection::Down,
            &mut graphics_writer,
            &mut check_writer,
            &mut current_board_wall_locations
        ));
        assert!(!detected_as_invalid_request_inner(
            BasicDirection::Left,
            &mut graphics_writer,
            &mut check_writer,
            &mut current_board_wall_locations
        ));
    }

    fn detected_as_invalid_request_inner(
        from_dir: BasicDirection,
        graphics_writer: &mut EventWriter<UpdateTileLocationGraphics>,
        check_writer: &mut EventWriter<CheckIfBoardIsSolved>,
        current_board_wall_locations: &mut ResMut<CurrentBoardWallLocations>
    ) -> bool {
        let mut board = generate_solved_board_inner(
            &BoardProperties::default(),
            current_board_wall_locations
        ).unwrap();
        board.ignore_player_input = false;
        let direction_check_outcome =
            move_tile_logic_inner(graphics_writer, check_writer, from_dir, 0, &mut board.clone());
        match direction_check_outcome {
            Err(TileMoveError::NoOccupiedTileInThatDirection(_)) => true,
            _ => false,
        }
    }
}
