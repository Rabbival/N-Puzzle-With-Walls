#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::*;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        
        .add_systems(PostStartup, expirimentation)
        .add_systems(Update, rotator_system)
        
        
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
        //costume basics
        .add_plugins((
            SystemSetsPlugin,
            EventPlugin,
            StatePlugin,
            ErrorHandlerPlugin,
        ))
        //costume
        .add_plugins((
            DataBasePlugin,
            InputPlugin,
            AssetLoaderPlugin,
            BoardPlugin,
            GraphicsPlugin,
            TileDictionaryPlugin,
        ));

    app.run();
}


use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::render::view::RenderLayers;
use bevy::render::camera::RenderTarget;

#[derive(Component)]
struct CubeInTheTexture;

fn expirimentation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);

    let image_handle = images.add(image);

    let cube_handle = meshes.add(Mesh::from(Cuboid { half_size: 2.0 * Vec3::ONE }));
    let cube_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.9, 0.9, 0.9),
        reflectance: 0.02,
        unlit: false,
        ..default()
    });

    // The cube that will be rendered to the texture.
    commands.spawn((
        PbrBundle {
            mesh: cube_handle,
            material: cube_material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        CubeInTheTexture,
        RenderLayers::layer(10),
    ));

    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        RenderLayers::layer(10),
    ));
    
    // Ui image to draw on
    commands.spawn(UiImage::new(image_handle));
}

/// Rotates the inner cube (first pass)
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<CubeInTheTexture>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.5 * time.delta_seconds());
        transform.rotate_z(1.3 * time.delta_seconds());
    }
}