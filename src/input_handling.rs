use crate::prelude::*;

#[derive(Resource, Default)]
pub struct CursorPosition {
    pub world_position: Vec2,
    pub over_ui: bool,
}

pub struct InputHandlingPlugin;

impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, (move_tile, update_cursor));
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

fn move_tile(
    mut commands: Commands,
    cursor_position: Res<CursorPosition>,
    mouse: Res<Input<MouseButton>>,
    mut target: Query<&GridLocation, With<TileType>>
) {
    if !mouse.pressed(MouseButton::Left) {
        return;
    }

    if let Some(location) = GridLocation::from_world(cursor_position.world_position) {
        if wall_grid.occupied(&location) {
            return;
        }
        
        //check if there's an empty space next to it (in straight line)
        //if there is, move it and update the grid
    }
}