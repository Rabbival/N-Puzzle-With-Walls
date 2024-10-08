use bevy::prelude::*;

pub const BIG_ATLAS_CELL_SQUARE_SIZE: f32 = 64.0;
pub const SMALL_ATLAS_CELL_SQUARE_SIZE: f32 = 32.0;

#[derive(Resource, Clone, Default)]
pub struct TileSpriteAtlas {
    pub atlas_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>
}

#[derive(Resource, Clone, Default)]
pub struct ArrowSpriteAtlas {
    pub atlas_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>
}

#[derive(Resource, Deref, DerefMut, Clone, Default)]
pub struct TileTextFont(pub Handle<Font>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (
            tile_sprite_atlas_setup, 
            arrow_sprite_atlas_setup,
            tile_text_font_setup
        ));
    }
}

fn tile_text_font_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font_handle = asset_server.load("FiraSans-Bold.ttf");
    commands.insert_resource(TileTextFont(font_handle));
}

fn tile_sprite_atlas_setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let image_handle = asset_server.load("tile_sprite_atlas.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(BIG_ATLAS_CELL_SQUARE_SIZE, BIG_ATLAS_CELL_SQUARE_SIZE),
        2,
        2,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TileSpriteAtlas {
        atlas_handle: texture_atlas_handle,
        image_handle
    });
}

fn arrow_sprite_atlas_setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
){
    let image_handle = asset_server.load("arrow_sprite_atlas.png");
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(SMALL_ATLAS_CELL_SQUARE_SIZE, SMALL_ATLAS_CELL_SQUARE_SIZE),
        2,
        2,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(ArrowSpriteAtlas {
        atlas_handle: texture_atlas_handle,
        image_handle
    });
}
