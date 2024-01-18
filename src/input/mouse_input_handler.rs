use crate::{
    costume_event::move_tile_event,
    output::{error_handler, print_to_console},
    prelude::*,
};

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
                .run_if(in_state(AppState::Game))
                .run_if(in_state(GameState::Regular))
                .chain()
                .in_set(InputSystemSets::InputListening),
        );
    }
}

fn update_cursor_in_game_world(
    mut cursor: ResMut<CursorPosition>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
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
    mut logic_event_writer: EventWriter<move_tile_event::SwitchTilesLogic>,
    mouse: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    game_board_query: Query<&TileBoard, (With<GameBoard>, Without<SolvedBoard>)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Err(input_error) = handle_mouse_click(
            &mut logic_event_writer,
            cursor_position.world_position,
            game_board_query.single(),
        ) {
            print_to_console::print_tile_move_error(input_error);
        }
    }
}

fn handle_mouse_click(
    logic_event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    cursor_position: Vec2,
    game_board: &TileBoard,
) -> Result<(), error_handler::TileMoveError> {
    if game_board.ignore_player_input {
        return Err(error_handler::TileMoveError::BoardFrozenToPlayer);
    }

    let grid_location_from_click = 
        GridLocation::from_world(&game_board.grid, cursor_position);
    let optional_occupied_tile_location ;
    match grid_location_from_click{
        Ok(grid_location) => 
            optional_occupied_tile_location=grid_location,
        Err(grid_error) => {
            return Err(error_handler::TileMoveError::GridError(grid_error));
        }
    }
    
    match game_board.is_tile_empty(&optional_occupied_tile_location) {
        Err(tile_board_error) =>
            return Err(error_handler::TileMoveError::TileBoardError(tile_board_error)),
        Ok(empty_tile) => {
            if empty_tile{
                return Err(error_handler::TileMoveError::PressedEmptySlot);
            }
        }
    }
    let occupied_tile_location = optional_occupied_tile_location;
    let optional_move_request =
        game_board.move_request_from_clicked_tile(&occupied_tile_location)?;
    match optional_move_request {
        None => Err(error_handler::TileMoveError::NoEmptyNeighbor),
        Some(move_request) => {
            logic_event_writer.send(move_tile_event::SwitchTilesLogic {
                move_neighbor_from_direction: move_request
                    .move_neighbor_from_direction
                    .unwrap(),
                empty_tile_index: move_request.empty_tile_index.unwrap(),
            });
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::board_building::solved_board_builder;

    use super::*;

    #[test]
    fn test_input_validation() {
        let mut app = App::new();
        app.add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_input_validation_inner);
        app.update();
    }

    fn test_input_validation_inner(
        mut event_writer: EventWriter<move_tile_event::SwitchTilesLogic>,
    ) {
        assert!(test_index_out_of_bound(
            Vec2::new(-100.0, -100.0),
            &mut event_writer
        ));
        assert!(test_index_out_of_bound(
            Vec2::new(
                BoardSize::default().to_grid_side_length() as f32 * ATLAS_CELL_SQUARE_SIZE,
                BoardSize::default().to_grid_side_length() as f32 * ATLAS_CELL_SQUARE_SIZE
            ),
            &mut event_writer
        ));
    }

    fn test_index_out_of_bound(
        position_to_check: Vec2,
        event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    ) -> bool {
        let mut board = TileBoard::default();
        board.ignore_player_input = false;
        let location_search_outcome = handle_mouse_click(event_writer, position_to_check, &board);
        match location_search_outcome {
            Err(error_handler::TileMoveError::GridError(_)) => true,
            _ => false,
        }
    }

    #[test]
    fn test_board_freezing() {
        let mut app = App::new();
        app.add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_board_freezing_inner);
        app.update();
    }

    fn test_board_freezing_inner(mut event_writer: EventWriter<move_tile_event::SwitchTilesLogic>) {
        assert!(test_frozen_board(&mut event_writer));
    }

    fn test_frozen_board(
        event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    ) -> bool {
        let location_validation_outcome = handle_mouse_click(
            event_writer,
            Vec2::default(),
            &TileBoard::default(), //locked by default
        );
        match location_validation_outcome {
            Err(TileMoveError::BoardFrozenToPlayer) => true,
            _ => false,
        }
    }

    #[test]
    fn test_valid_location() {
        let mut app = App::new();
        app.add_event::<move_tile_event::SwitchTilesLogic>()
            .add_systems(Update, test_valid_location_inner);
        app.update();
    }

    fn test_valid_location_inner(mut event_writer: EventWriter<move_tile_event::SwitchTilesLogic>) {
        assert!(test_no_tile_in_cell(&mut event_writer));
        assert!(test_empty_slot(&mut event_writer));
        assert!(test_no_empty_neighbor(&mut event_writer));
    }

    fn test_no_tile_in_cell(
        event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    ) -> bool {
        let mut board = TileBoard::default();
        board.ignore_player_input = false;
        let location_validation_outcome = handle_mouse_click(event_writer, Vec2::default(), &board);
        match location_validation_outcome {
            Err(TileMoveError::TileBoardError(TileBoardError::NoTileInCell(_))) => true,
            _ => false,
        }
    }

    fn test_empty_slot(event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>) -> bool {
        let mut board = TileBoard::default();
        board.ignore_player_input = false;
        board.set(
            &GridLocation::new(0, 0),
            Tile {
                index: 0,
                tile_type: TileType::Empty,
            },
        ).unwrap();
        let location_validation_outcome = handle_mouse_click(event_writer, Vec2::default(), &board);
        match location_validation_outcome {
            Err(TileMoveError::PressedEmptySlot) => true,
            _ => false,
        }
    }

    fn test_no_empty_neighbor(
        event_writer: &mut EventWriter<move_tile_event::SwitchTilesLogic>,
    ) -> bool {
        let mut board: TileBoard =
            solved_board_builder::generate_solved_board_inner(
                &BoardProperties::default(),
                &mut DataBaseManager::default()
            ).unwrap();
        board.ignore_player_input = false;

        //fill all empties
        let empty_tile_locations = board.empty_tile_locations.clone();
        for empty_tile_location in &empty_tile_locations {
            board.set(
                empty_tile_location,
                Tile {
                    index: 0,
                    tile_type: TileType::Numbered,
                },
            ).unwrap();
        }

        let location_validation_outcome = handle_mouse_click(event_writer, Vec2::default(), &board);
        match location_validation_outcome {
            Err(TileMoveError::NoEmptyNeighbor) => true,
            _ => false,
        }
    }
}
