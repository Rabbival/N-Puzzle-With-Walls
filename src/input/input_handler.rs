use crate::{prelude::*, logic::board_manager};

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
    pub over_ui: bool,
}

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (update_cursor, move_tile_input).chain());
    }
}

pub fn update_cursor(
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
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
    board_grid: Res<Grid>,
) {
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
        board_manager::move_tile_logic(location, board_grid);
    }
}