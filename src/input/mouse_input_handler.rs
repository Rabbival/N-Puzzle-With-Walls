use crate::{prelude::*, output::{print_to_console, error_handler}, logic::board_manager};

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2
}

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CursorPosition>()
            .add_systems(Update, (update_cursor, listen_for_mouse_click).chain());
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

fn listen_for_mouse_click(
    mouse: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut game_board_query: Query<&mut Board<Tile>, (With<GameBoard>, Without<SolvedBoard>)>,
    solved_board_query: Query<&Board<Tile>, (With<SolvedBoard>, Without<GameBoard>)>,
    tiles: Query<&mut Transform, With<Tile>>
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Err(input_error) = 
        handle_mouse_click(
                cursor_position.world_position, 
                game_board_query.single_mut().into_inner(),
                solved_board_query.single(),
                Some(tiles)
            )
        {
            print_to_console::print_input_error(input_error);
        }
    }
}

/// tile query optional for the sake of testing.
fn handle_mouse_click(
    cursor_position: Vec2,
    game_board: &mut Board<Tile>,
    solved_board: &Board<Tile>,
    optional_tiles: Option<Query<&mut Transform, With<Tile>>>
) -> Result<(), error_handler::TileMoveError>
{
    if let Some(optional_occupied_tile_location) = GridLocation::from_world(cursor_position) {
        if game_board.ignore_player_input{
            return Err(error_handler::TileMoveError::BoardFrozenToPlayer(String::from("board locked")));
        }
        if !game_board.occupied(&optional_occupied_tile_location) {
            return Err(error_handler::TileMoveError::PressedEmptySlot(String::from("pressed an empty slot")));
        }
        let occupied_tile_location=optional_occupied_tile_location;
        let optional_empty_neighbor_location= 
            game_board.get_empty_neighbor(&occupied_tile_location);
        if let None=optional_empty_neighbor_location{
            return Err(error_handler::TileMoveError::NoEmptyNeighbor(String::from("no empty neighbor")));
        }
        let empty_neighbor_location=optional_empty_neighbor_location.unwrap();

        if let Some(tiles) = optional_tiles{
            return board_manager::move_tile_logic(
                occupied_tile_location, 
                empty_neighbor_location,
                game_board, 
                solved_board,
                tiles
            )
        }
        Ok(()) //only here for the sake of testing, there will always be tiles.
    }else{
        Err(error_handler::TileMoveError::IndexOutOfGridBounds(String::from("index out of grid bounds!")))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_validation() {
        assert!(test_index_out_of_bound(Vec2::new(-100.0, -100.0)));
        assert!(test_index_out_of_bound(Vec2::new(WINDOW_SIZE, WINDOW_SIZE)));
    }

    fn test_index_out_of_bound(position_to_check: Vec2)-> bool{
        let location_search_outcome=
            handle_mouse_click(
                position_to_check, 
                &mut Board::<Tile>::default(),
                &Board::<Tile>::default(),
                None
            );
        match location_search_outcome{
            Err(error_handler::TileMoveError::IndexOutOfGridBounds(_))=> true,
            _ => false
        }
    }

    #[test]
    fn test_board_freezing() {
        assert!(test_frozen_board());
    }

    fn test_frozen_board()-> bool{
        let location_validation_outcome=
            handle_mouse_click(
                Vec2::default(), 
                &mut Board::<Tile>::default(), //locked be default
                &Board::<Tile>::default(),
                None
            );
        match location_validation_outcome{
            Err(TileMoveError::BoardFrozenToPlayer(_))=> true,
            _ => false
        }
    }

    #[test]
    fn test_valid_location(){
        assert!(test_empty_slot());
        assert!(test_no_empty_neighbor());
    }

    fn test_empty_slot()-> bool{
        let mut board=Board::<Tile>::default();
        board.ignore_player_input=false;
        let location_validation_outcome=
            handle_mouse_click(
                Vec2::default(), 
                &mut board,
                &Board::<Tile>::default(),
                None
            );
        match location_validation_outcome{
            Err(TileMoveError::PressedEmptySlot(_))=> true,
            _ => false
        }
    }

    fn test_no_empty_neighbor()-> bool{
        let mut board=board_manager::generate_solved_board();
        board.ignore_player_input=false;
        let empty_tile_location=board.empty_tile_location;
        board[&empty_tile_location]=Tile::new(Some(16));
        let location_validation_outcome=
            handle_mouse_click(
                Vec2::default(), 
                &mut board,
                &Board::<Tile>::default(),
                None
            );
        match location_validation_outcome{
            Err(TileMoveError::NoEmptyNeighbor(_))=> true,
            _ => false
        }
    }
}