use bevy::render::view::RenderLayers;
use enum_iterator::all;
use crate::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_cameras)
            .add_systems(OnEnter(GameState::GameBoardGenerated),
                adjust_main_camera_zoom_to_new_settings
            );
    }
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn((
       MainCamera,
       Camera2dBundle{
            camera: Camera{
                order: 0,
                ..default()
            },
           ..default()
        },
       RenderLayers::layer(0),
    ));

    for loader_slot in all::<LoaderScreenSlot>(){
        spawn_loader_slot_preview_camera(loader_slot, &mut commands)
    }
}

fn spawn_loader_slot_preview_camera(loader_slot: LoaderScreenSlot, commands: &mut Commands){
    let loader_slot_ownership_tag = LoaderSlotOwnershipTag(Some(loader_slot));
    commands.spawn((
        Camera2dBundle {
            camera: Camera{
                //TODO: set target to slot's ui image
                order: loader_slot_ownership_tag.to_camera_order(),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(loader_slot_ownership_tag.to_render_layer()),
        loader_slot_ownership_tag
    ));
}

fn adjust_main_camera_zoom_to_new_settings(
    mut camera_query: Query<
        (&mut Transform, &mut OrthographicProjection, &LoaderSlotOwnershipTag), 
        With<Camera2d>
    >,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
) {
    for (
        mut camera_transform,
        mut camera_projection,
        loader_slot_tag
    )
        in &mut camera_query
    {
        if loader_slot_tag.0.is_none(){
            set_camera_zoom_by_grid_side_length(
                applied_board_props_query.single().size.to_grid_side_length(),
                camera_transform.as_mut(),
                camera_projection.as_mut(),
            );
            break;
        }
    }
}

fn set_camera_zoom_by_grid_side_length(
    grid_side_length: u8,
    camera_transform: &mut Transform,
    camera_projection: &mut OrthographicProjection
){
    let new_camera_zoom = 
        grid_side_length as f32 * ATLAS_CELL_SQUARE_SIZE / BOARD_SIZE_IN_PIXELS;
    
    camera_transform.translation.x =
        (grid_side_length - 1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera_transform.translation.y =
        -1.0 * (grid_side_length - 1) as f32 / 2.0 * ATLAS_CELL_SQUARE_SIZE;
    camera_projection.scale = new_camera_zoom;
}