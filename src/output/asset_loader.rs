use bevy::prelude::*;

pub const ATLAS_CELL_SQUARE_SIZE:f32=32.0;

#[derive(Resource, Deref, DerefMut, Clone, Default)]
pub struct SpriteAtlas(pub Handle<TextureAtlas>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
    fn build(&self, app: &mut App){
        app
            .init_resource::<SpriteAtlas>()
            .add_systems(Startup, sprite_atlas_setup);
    }
}

fn sprite_atlas_setup(
    atlas: ResMut<SpriteAtlas>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, 
            Vec2::new(
                ATLAS_CELL_SQUARE_SIZE, 
                ATLAS_CELL_SQUARE_SIZE
            )
            , 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    atlas.clone().0=texture_atlas_handle;
}