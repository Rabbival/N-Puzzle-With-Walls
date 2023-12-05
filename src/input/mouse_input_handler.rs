use crate::{prelude::*, logic::board_manager, output::error_handler};

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2
}

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (update_cursor, move_tile_input).chain());
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

fn move_tile_input(
    mouse: Res<Input<MouseButton>>,
    cursor_position: Res<CursorPosition>,
    mut board_query: Query<&mut Board, With<GameBoard>>
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if let Err(input_err) = 
        forward_location_to_board_manager(
            cursor_position.world_position, 
            board_query.single_mut().into_inner())
    {
        match input_err{
            InputHandlerError::BoardLocked(message)=>{
                warn!(message);
            },
            InputHandlerError::NoEmptyNeighbor(message)=>{
                warn!(message);
            },
            InputHandlerError::PressedEmptySlot(message)=>{
                warn!(message);
            },
            InputHandlerError::IndexOutOfGridBounds(message)=>{
                error!(message);
            }
        }
    };
}

fn forward_location_to_board_manager(
    cursor_position: Vec2,
     board: &mut Board,
) -> Result<(), error_handler::InputHandlerError>
{
    if let Some(location) = GridLocation::from_world(cursor_position) {
        game_log(GameLog::TileClicked(location));
        return board_manager::move_tile_logic(location, board);
    }else{
        Err(InputHandlerError::IndexOutOfGridBounds(String::from("index out of grid bounds!")))
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
                &mut Board::default()
            );
        match location_search_outcome{
                Err(InputHandlerError::IndexOutOfGridBounds(_))=> true,
                _ => false
            }
    }
}