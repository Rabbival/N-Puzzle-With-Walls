use bevy::prelude::*;

pub const ATLAS_CELL_SQUARE_SIZE: f32 = 32.0;

#[derive(Resource, Deref, DerefMut, Clone, Default)]
pub struct SpriteAtlas(pub Handle<TextureAtlas>);

#[derive(Resource, Deref, DerefMut, Clone, Default)]
pub struct TileTextFont(pub Handle<Font>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (sprite_atlas_setup, tile_text_font_setup));
    }
}

fn tile_text_font_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("FiraSans-Bold.ttf");
    commands.insert_resource(TileTextFont(font_handle));
}

fn sprite_atlas_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprite_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(ATLAS_CELL_SQUARE_SIZE, ATLAS_CELL_SQUARE_SIZE),
        4,
        4,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteAtlas(texture_atlas_handle));
}
