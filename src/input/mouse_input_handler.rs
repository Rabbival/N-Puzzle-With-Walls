use crate::{prelude::*, logic::board_manager, output::error_handler};

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
    pub over_ui: bool,
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
    board: ResMut<Board>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }
    if let Err(input_err) = 
        forward_location_to_board_manager(cursor_position, board)
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
    cursor_position: Res<CursorPosition>,
    board: ResMut<Board>,
) -> Result<(), error_handler::InputHandlerError>
{
    if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
        game_log(GameLog::TileClicked(location));
        return board_manager::move_tile_logic(location, board);
    }else{
        Err(InputHandlerError::IndexOutOfGridBounds(String::from("index out of grid bounds!")))
    }
}