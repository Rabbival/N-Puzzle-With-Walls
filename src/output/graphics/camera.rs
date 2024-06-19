use bevy::render::camera::RenderTarget;
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
            )
            .add_systems(Update, adjust_loader_slot_cameras_zoom
                .after(show_currently_displayed_saved_layouts_screen));
    }
}

fn spawn_cameras(mut commands: Commands, loader_slot_query: Query<(&UiImage, &LayoutPreviewNode)>) {
    commands.spawn((
       MainCamera,
       Camera2dBundle{
            camera: Camera{
                order: (all::<LoaderScreenSlot>().count()+1) as isize,
                ..default()
            },
           ..default()
        },
       RenderLayers::layer(0),
    ));

    for (ui_image, preview_node) in &loader_slot_query{
        spawn_loader_slot_preview_camera(preview_node.0, ui_image.texture.clone(), &mut commands);
    }
}

fn spawn_loader_slot_preview_camera(
    loader_slot: LoaderScreenSlot,
    image_handle: Handle<Image>,
    commands: &mut Commands
){
    let loader_slot_ownership_tag = LoaderSlotOwnershipTag(Some(loader_slot));
    commands.spawn((
        Camera2dBundle {
            camera: Camera{
                target: RenderTarget::Image(image_handle),
                order: loader_slot.to_camera_order(),
                ..default()
            },
            ..default()
        },
        RenderLayers::layer(loader_slot_ownership_tag.to_render_layer()),
        loader_slot_ownership_tag
    ));
}

fn adjust_main_camera_zoom_to_new_settings(
    mut main_camera_query: Query<
        (&mut Transform, &mut OrthographicProjection), 
        (With<Camera2d>, Without<LoaderSlotOwnershipTag>)
    >,
    applied_board_props_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
) {
    let (mut camera_transform, mut camera_projection)
        = main_camera_query.single_mut();
    set_camera_zoom_by_grid_side_length(
        false,
        applied_board_props_query.single().size.to_grid_side_length(),
        camera_transform.as_mut(),
        camera_projection.as_mut(),
    );
}

fn adjust_loader_slot_cameras_zoom(
    mut event_reader: EventReader<LoaderSlotSetEvent>,
    mut loader_slot_cameras_query: Query<
        (&mut Transform, &mut OrthographicProjection, &LoaderSlotOwnershipTag),
        With<Camera2d>
    >,
    domain_board_query: Query<&DomainBoard>
) {
    for loader_slot_set_request in event_reader.read(){
        for (
            mut camera_transform,
            mut camera_projection,
            loader_slot_ownership_tag
        ) in &mut loader_slot_cameras_query
        {
            if let Some(loader_slot) = loader_slot_ownership_tag.0 {
                if loader_slot == loader_slot_set_request.slot_to_set {
                    if let Ok(domain_board) =
                        domain_board_query.get(loader_slot_set_request.layout_entity)
                    {
                        set_camera_zoom_by_grid_side_length(
                            true,
                            domain_board.board_props.size.to_grid_side_length(),
                            camera_transform.as_mut(),
                            camera_projection.as_mut(),
                        );
                    }
                }
            }
        }

    }
}

fn set_camera_zoom_by_grid_side_length(
    ui_preview_camera: bool,
    grid_side_length: u8,
    camera_transform: &mut Transform,
    camera_projection: &mut OrthographicProjection
){
    let mut new_camera_zoom =
        grid_side_length as f32 * BIG_ATLAS_CELL_SQUARE_SIZE / BOARD_SIZE_IN_PIXELS;
    if ui_preview_camera{
        new_camera_zoom *= 1.2;
    }
    
    camera_transform.translation.x =
        (grid_side_length - 1) as f32 / 2.0 * BIG_ATLAS_CELL_SQUARE_SIZE;
    camera_transform.translation.y =
        -1.0 * (grid_side_length - 1) as f32 / 2.0 * BIG_ATLAS_CELL_SQUARE_SIZE;
    camera_projection.scale = new_camera_zoom;
}