use crate::prelude::*;

pub struct BoardManagerPlugin;

impl Plugin for BoardManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (listen_for_tile_shift_requests, check_if_solved)
                    .chain()
                    .in_set(InputSystemSets::InputHandling),
                listen_for_board_lock_change_requests.in_set(InputSystemSets::LateChanges),
            ),
        );
    }
}

fn listen_for_board_lock_change_requests(
    mut event_reader: EventReader<SetGameBoardLock>,
    mut game_board_query: Query<&mut TileBoard, With<GameBoard>>,
) {
    for lock_change_request in event_reader.read() {
        game_board_query.single_mut().ignore_player_input = lock_change_request.0;
    }
}

/// graphics switched before logic for the sake of graphics function readability
fn listen_for_tile_shift_requests(
    mut graphics_event_writer: EventWriter<UpdateTileLocationGraphics>,
    mut check_if_board_is_solved_writer: EventWriter<CheckIfBoardIsSolved>,
    mut logic_event_reader: EventReader<ShiftTilesInDirectionRequest>,
    mut game_board_query: Query<&mut TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
) {
    for shift_tiles_request in logic_event_reader.read() {
        if let Err(move_error) = listen_for_tile_shift_requests_inner(
            &mut graphics_event_writer,
            &mut check_if_board_is_solved_writer,
            shift_tiles_request,
            &mut game_board_query.single_mut(),
        ) {
            print_tile_move_error(move_error);
        }
    }
}

/// graphics switched before logic for the sake of graphics function readability
fn listen_for_tile_shift_requests_inner(
    graphics_event_writer: &mut EventWriter<UpdateTileLocationGraphics>,
    check_if_board_is_solved_writer: &mut EventWriter<CheckIfBoardIsSolved>,
    shift_tiles_request: &ShiftTilesInDirectionRequest,
    game_board: &mut TileBoard,
) -> Result<(), TileMoveError> {
    if game_board.ignore_player_input {
        return Err(TileMoveError::BoardFrozenToPlayer);
    }
    let empty_tile_index = shift_tiles_request.empty_tile_index;
    let direction_to_move_from = shift_tiles_request.direction_to_shift_from;

    for _ in 0..shift_tiles_request.steps_count {
        let empty_tile_neighbors = game_board.get_direct_neighbors_of_empty(empty_tile_index);
        match empty_tile_neighbors.get(&direction_to_move_from) {
            Some(occupied_tile_original_location) => {
                switch_between_tiles(
                    graphics_event_writer,
                    game_board,
                    occupied_tile_original_location,
                    empty_tile_index,
                )?;
            }
            None => {
                return Err(TileMoveError::NoOccupiedTileInThatDirection(
                    direction_to_move_from,
                ))
            }
        }
    }

    check_if_board_is_solved_writer.send(CheckIfBoardIsSolved);
    Ok(())
}

fn switch_between_tiles(
    graphics_event_writer: &mut EventWriter<UpdateTileLocationGraphics>,
    game_board: &mut TileBoard,
    occupied_tile_original_location: &GridLocation,
    empty_tile_index: usize,
) -> Result<(), TileMoveError> {
    let occupied_tile = get_occupied_tile_if_valid(occupied_tile_original_location, game_board)?;
    let empty_tile_original_location = *game_board.get_empty_tile_location(empty_tile_index);
    let empty_tile = *game_board.try_get_empty_tile(empty_tile_index)?;
    game_board.swap_tiles_by_location(
        &empty_tile_original_location,
        &occupied_tile_original_location,
    )?;
    game_log(GameLog::TilesMoved(
        &occupied_tile,
        &empty_tile_original_location,
    ));

    graphics_event_writer.send(UpdateTileLocationGraphics {
        tile: occupied_tile,
        new_location: empty_tile_original_location,
    });
    graphics_event_writer.send(UpdateTileLocationGraphics {
        tile: empty_tile,
        new_location: *occupied_tile_original_location,
    });

    Ok(())
}

fn get_occupied_tile_if_valid(
    occupied_tile_original_location: &GridLocation,
    game_board: &TileBoard,
) -> Result<Tile, TileMoveError> {
    match game_board.get(occupied_tile_original_location)? {
        Some(occupied_tile) => match occupied_tile.tile_type {
            TileType::Wall => Err(TileMoveError::TriedToSwitchWithAWall),
            _ => Ok(*occupied_tile),
        },
        None => Err(TileMoveError::TileBoardError(TileBoardError::NoTileInCell(
            *occupied_tile_original_location,
        ))),
    }
}

fn check_if_solved(
    mut check_if_board_is_solved_listener: EventReader<CheckIfBoardIsSolved>,
    mut set_game_state_to_victory: ResMut<NextState<GameState>>,
    game_board_query: Query<&TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&TileBoard, (With<SolvedBoard>, Without<GameBoard>)>,
) {
    'request_check: for _check_request in check_if_board_is_solved_listener.read() {
        let game_board_iter = game_board_query.single().grid.iter();
        let solved_board_iter = solved_board_query.single().grid.iter();
        for ((_, game_tile), (_, solved_tile)) in game_board_iter.zip(solved_board_iter) {
            if game_tile.tile_type == TileType::Empty && solved_tile.tile_type == TileType::Empty {
                continue;
            }
            if game_tile != solved_tile {
                continue 'request_check;
            }
        }
        set_game_state_to_victory.set(GameState::Victory);
        game_log(GameLog::Victory);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_request() {
        let mut app = App::new();
        app.add_event::<UpdateTileLocationGraphics>()
            .add_event::<CheckIfBoardIsSolved>()
            .add_systems(Update, test_valid_request_inner);
        app.update();
    }

    fn test_valid_request_inner(
        mut graphics_writer: EventWriter<UpdateTileLocationGraphics>,
        mut check_writer: EventWriter<CheckIfBoardIsSolved>,
    ) {
        assert!(!detected_as_invalid_request_inner(
            BasicDirection::Up,
            &mut graphics_writer,
            &mut check_writer,
        ));
        assert!(detected_as_invalid_request_inner(
            BasicDirection::Right,
            &mut graphics_writer,
            &mut check_writer,
        ));
        assert!(detected_as_invalid_request_inner(
            BasicDirection::Down,
            &mut graphics_writer,
            &mut check_writer,
        ));
        assert!(!detected_as_invalid_request_inner(
            BasicDirection::Left,
            &mut graphics_writer,
            &mut check_writer,
        ));
    }

    fn detected_as_invalid_request_inner(
        from_dir: BasicDirection,
        graphics_writer: &mut EventWriter<UpdateTileLocationGraphics>,
        check_writer: &mut EventWriter<CheckIfBoardIsSolved>,
    ) -> bool {
        let mut tile_board = TileBoard::default();
        let tile_shift_request = ShiftTilesInDirectionRequest {
            empty_tile_index: 0,
            direction_to_shift_from: from_dir,
            steps_count: 1,
        };
        generate_solved_board_inner(&BoardProperties::default(), &mut tile_board).unwrap();
        tile_board.ignore_player_input = false;
        let direction_check_outcome = listen_for_tile_shift_requests_inner(
            graphics_writer,
            check_writer,
            &tile_shift_request,
            &mut tile_board,
        );
        matches!(
            direction_check_outcome,
            Err(TileMoveError::NoOccupiedTileInThatDirection(_))
        )
    }
}
