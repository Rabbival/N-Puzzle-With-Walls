use crate::{prelude::*, logic::board_manager, output::error_handler};

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2
}

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorPosition>()
            .add_systems(Update, (update_cursor, move_tile_mouse_input).chain());
    }
}

fn update_cursor(
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

fn move_tile_mouse_input(
    mouse: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut game_board_query: Query<&mut Board, (With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&Board, (With<SolvedBoard>, Without<GameBoard>)>,
    tiles: Query<&mut Transform, With<Tile>>
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if let Err(input_error) = 
        forward_location_to_board_manager(
            cursor_position.world_position, 
            game_board_query.single_mut().into_inner(),
            solved_board_query.single(),
            Some(tiles)
        )
    {
        print_input_error(input_error);
    }
}

/// Had to make the query optional for the sake of testing,
/// for the sake of the game- it's always there
fn forward_location_to_board_manager(
    cursor_position: Vec2,
    game_board: &mut Board,
    solved_board: &Board,
    optional_tiles: Option<Query<&mut Transform, With<Tile>>>
) -> Result<(), error_handler::TileMoveError>
{
    if let Some(optional_occupied_tile_location) = GridLocation::from_world(cursor_position) {
        game_log(GameLog::TileClicked(optional_occupied_tile_location));
        return board_manager::move_tile_logic(
            optional_occupied_tile_location, 
            optional_tiles,
            game_board, 
            solved_board
        );
    }else{
        Err(TileMoveError::IndexOutOfGridBounds(String::from("index out of grid bounds!")))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_out_of_bounds() {
        assert!(test_index_out_of_bound(Vec2::new(-100.0, -100.0)));
        assert!(test_index_out_of_bound(Vec2::new(WINDOW_SIZE, WINDOW_SIZE)));
    }

    fn test_index_out_of_bound(position_to_check: Vec2)-> bool{
        let location_search_outcome=
            forward_location_to_board_manager(
                position_to_check, 
                &mut Board::default(),
                &Board::default(),
                None
            );
        match location_search_outcome{
                Err(TileMoveError::IndexOutOfGridBounds(_))=> true,
                _ => false
            }
    }
}