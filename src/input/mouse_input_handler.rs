use crate::prelude::*;

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
}

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>().add_systems(
            Update,
            (update_cursor_in_game_world, listen_for_mouse_click_in_game)
                .run_if(in_state(AppState::Game).and_then(in_state(GameState::Regular)))
                .chain()
                .in_set(InputSystemSets::InputListening),
        );
    }
}

fn update_cursor_in_game_world(
    mut cursor: ResMut<CursorPosition>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, transform) = camera.single();

    if let Some(screen_position) = window.cursor_position() {
        let world_position = camera
            .viewport_to_world(transform, screen_position)
            .unwrap()
            .origin
            .truncate();
        cursor.world_position = world_position;
    }
}

fn listen_for_mouse_click_in_game(
    mut logic_event_writer: EventWriter<ShiftTilesInDirectionRequest>,
    mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    game_board_query: Query<&TileBoard, With<GameBoard>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Err(input_error) = handle_mouse_click(
            &mut logic_event_writer,
            &mut multiple_empty_tiles_choice_manager,
            cursor_position.world_position,
            game_board_query.single(),
        ) {
            print_tile_move_error(input_error);
        }
    }
}

fn handle_mouse_click(
    tile_switch_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
    multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    cursor_position: Vec2,
    game_board: &TileBoard,
) -> Result<(), TileMoveError> {
    if game_board.ignore_player_input {
        return Err(TileMoveError::BoardFrozenToPlayer);
    }
    let clicked_grid_location = clicked_location_to_grid_location(cursor_position, game_board)?;

    if multiple_empty_tiles_choice_manager.choice_pending {
        handle_request_when_empty_choice_pending(
            tile_switch_event_writer,
            multiple_empty_tiles_choice_manager,
            clicked_grid_location,
            game_board,
        );
    } else {
        handle_request_no_choice_pending(
            tile_switch_event_writer,
            multiple_empty_tiles_choice_manager,
            clicked_grid_location,
            game_board,
        )?;
    }

    Ok(())
}

fn clicked_location_to_grid_location(
    cursor_position: Vec2,
    game_board: &TileBoard,
) -> Result<GridLocation, TileMoveError> {
    let clicked_optional_grid_location =
        GridLocation::from_world(&game_board.grid, cursor_position);
    match clicked_optional_grid_location {
        Err(grid_error) => Err(TileMoveError::GridError(grid_error)),
        Ok(grid_location) => Ok(grid_location),
    }
}

fn handle_request_when_empty_choice_pending(
    tile_switch_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
    multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    clicked_grid_location: GridLocation,
    game_board: &TileBoard,
) {
    multiple_empty_tiles_choice_manager.choice_pending = false;
    if let Some(empty_tiles) =
        &multiple_empty_tiles_choice_manager.possible_empty_tiles_locations_and_directions
    {
        for tile_in_direct_line in empty_tiles {
            send_shift_tiles_in_direction_request_if_empty_tile_was_clicked(
                tile_switch_event_writer,
                clicked_grid_location,
                tile_in_direct_line,
                game_board,
            );
        }
    }
}

fn send_shift_tiles_in_direction_request_if_empty_tile_was_clicked(
    tile_switch_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
    clicked_grid_location: GridLocation,
    tile_in_direct_line: &TileInDirectLine,
    game_board: &TileBoard,
) {
    if let Some(shift_request) = tile_in_direct_line.try_to_tiles_shift_request() {
        let empty_tile_location =
            game_board.get_empty_tile_location(shift_request.empty_tile_index);
        if *empty_tile_location == clicked_grid_location {
            tile_switch_event_writer.send(shift_request);
        }
    }
}

fn handle_request_no_choice_pending(
    tile_switch_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
    multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    clicked_grid_location: GridLocation,
    game_board: &TileBoard,
) -> Result<(), TileMoveError> {
    let empty_tiles_in_direct_line =
        get_empty_tiles_in_direct_line_if_numbered(clicked_grid_location, game_board)?;
    match empty_tiles_in_direct_line.len() {
        0 => Err(TileMoveError::NoEmptyTileInDirectLine),
        1 => {
            if let Some(shift_request) = empty_tiles_in_direct_line
                .first()
                .unwrap()
                .try_to_tiles_shift_request()
            {
                tile_switch_event_writer.send(shift_request);
            }
            Ok(())
        }
        _ => {
            if game_board.tiletype_in_location(&clicked_grid_location)? == TileType::Numbered {
                **multiple_empty_tiles_choice_manager = MultipleEmptyTilesChoiceManager {
                    choice_pending: true,
                    possible_empty_tiles_locations_and_directions: Some(empty_tiles_in_direct_line),
                };
            }
            Ok(())
        }
    }
}

fn get_empty_tiles_in_direct_line_if_numbered(
    tile_location: GridLocation,
    game_board: &TileBoard,
) -> Result<Vec<TileInDirectLine>, TileMoveError> {
    let numbered_tile_location = match game_board.is_tile_empty(&tile_location) {
        Err(tile_board_error) => Err(TileMoveError::TileBoardError(tile_board_error)),
        Ok(empty_tile) => {
            if empty_tile {
                Err(TileMoveError::PressedEmptySlot)
            } else {
                Ok(tile_location)
            }
        }
    };

    Ok(game_board.get_empty_tiles_in_direct_line(&numbered_tile_location?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validation() {
        let mut app = App::new();
        app.add_event::<SwitchTilesLogic>()
            .init_resource::<MultipleEmptyTilesChoiceManager>()
            .add_systems(Update, test_input_validation_inner);
        app.update();
    }

    fn test_input_validation_inner(
        mut tile_logic_event_writer: EventWriter<ShiftTilesInDirectionRequest>,
        mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>,
    ) {
        assert!(test_index_out_of_bound(
            Vec2::new(-100.0, -100.0),
            &mut tile_logic_event_writer,
            &mut multiple_empty_tiles_choice_manager
        ));
        assert!(test_index_out_of_bound(
            Vec2::new(
                BoardSize::default().to_grid_side_length() as f32 * BIG_ATLAS_CELL_SQUARE_SIZE,
                BoardSize::default().to_grid_side_length() as f32 * BIG_ATLAS_CELL_SQUARE_SIZE
            ),
            &mut tile_logic_event_writer,
            &mut multiple_empty_tiles_choice_manager
        ));
    }

    fn test_index_out_of_bound(
        position_to_check: Vec2,
        tile_logic_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
        multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    ) -> bool {
        let board = TileBoard {
            ignore_player_input: false,
            ..default()
        };
        let location_search_outcome = handle_mouse_click(
            tile_logic_event_writer,
            multiple_empty_tiles_choice_manager,
            position_to_check,
            &board,
        );
        matches!(location_search_outcome, Err(TileMoveError::GridError(_)))
    }

    #[test]
    fn test_board_freezing() {
        let mut app = App::new();
        app.add_event::<ShiftTilesInDirectionRequest>()
            .init_resource::<MultipleEmptyTilesChoiceManager>()
            .add_systems(Update, test_board_freezing_inner);
        app.update();
    }

    fn test_board_freezing_inner(
        mut tile_logic_event_writer: EventWriter<ShiftTilesInDirectionRequest>,
        mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>,
    ) {
        assert!(test_frozen_board(
            &mut tile_logic_event_writer,
            &mut multiple_empty_tiles_choice_manager
        ));
    }

    fn test_frozen_board(
        tile_logic_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
        multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    ) -> bool {
        let location_validation_outcome = handle_mouse_click(
            tile_logic_event_writer,
            multiple_empty_tiles_choice_manager,
            Vec2::default(),
            &TileBoard::default(), //locked by default
        );
        matches!(
            location_validation_outcome,
            Err(TileMoveError::BoardFrozenToPlayer)
        )
    }

    #[test]
    fn test_valid_location() {
        let mut app = App::new();
        app.add_event::<ShiftTilesInDirectionRequest>()
            .init_resource::<MultipleEmptyTilesChoiceManager>()
            .add_systems(Update, test_valid_location_inner);
        app.update();
    }

    fn test_valid_location_inner(
        mut tile_logic_event_writer: EventWriter<ShiftTilesInDirectionRequest>,
        mut multiple_empty_tiles_choice_manager: ResMut<MultipleEmptyTilesChoiceManager>,
    ) {
        assert!(test_no_tile_in_cell(
            &mut tile_logic_event_writer,
            &mut multiple_empty_tiles_choice_manager
        ));
        assert!(test_empty_slot(
            &mut tile_logic_event_writer,
            &mut multiple_empty_tiles_choice_manager
        ));
        assert!(test_no_empty_neighbor(
            &mut tile_logic_event_writer,
            &mut multiple_empty_tiles_choice_manager
        ));
    }

    fn test_no_tile_in_cell(
        tile_logic_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
        multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    ) -> bool {
        let board = TileBoard {
            ignore_player_input: false,
            ..default()
        };
        let location_validation_outcome = handle_mouse_click(
            tile_logic_event_writer,
            multiple_empty_tiles_choice_manager,
            Vec2::default(),
            &board,
        );
        matches!(
            location_validation_outcome,
            Err(TileMoveError::TileBoardError(TileBoardError::NoTileInCell(
                _
            )))
        )
    }

    fn test_empty_slot(
        tile_logic_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
        multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    ) -> bool {
        let mut board = TileBoard {
            ignore_player_input: false,
            ..default()
        };
        board
            .set(
                &GridLocation::new(0, 0),
                Tile {
                    index: 0,
                    tile_type: TileType::Empty,
                },
            )
            .unwrap();
        let location_validation_outcome = handle_mouse_click(
            tile_logic_event_writer,
            multiple_empty_tiles_choice_manager,
            Vec2::default(),
            &board,
        );
        matches!(
            location_validation_outcome,
            Err(TileMoveError::PressedEmptySlot)
        )
    }

    fn test_no_empty_neighbor(
        tile_logic_event_writer: &mut EventWriter<ShiftTilesInDirectionRequest>,
        multiple_empty_tiles_choice_manager: &mut ResMut<MultipleEmptyTilesChoiceManager>,
    ) -> bool {
        let mut tile_board = TileBoard::default();
        generate_solved_board_inner(&BoardProperties::default(), &mut tile_board).unwrap();
        tile_board.ignore_player_input = false;

        //fill all empties
        let empty_tile_locations = tile_board.empty_tile_locations.clone();
        for empty_tile_location in &empty_tile_locations {
            tile_board
                .set(
                    empty_tile_location,
                    Tile {
                        index: 0,
                        tile_type: TileType::Numbered,
                    },
                )
                .unwrap();
        }

        let location_validation_outcome = handle_mouse_click(
            tile_logic_event_writer,
            multiple_empty_tiles_choice_manager,
            Vec2::default(),
            &tile_board,
        );
        matches!(
            location_validation_outcome,
            Err(TileMoveError::NoEmptyTileInDirectLine)
        )
    }
}
