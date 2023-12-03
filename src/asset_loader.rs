use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SpriteAtlas(Handle<TextureAtlas>);

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup,add_sprite_to_indexable::<TileSprite>);
    }
}

pub enum TileSprite{
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Empty
}

impl IndexableSprite for TileSprite {
    type AtlasHandleWrapper = SpriteAtlas;
    fn index(&self) -> usize {
        match self {
            TileSprite::One=>0,
            TileSprite::Two=>1,
            TileSprite::Three=>2,
            TileSprite::Four=>3,
            TileSprite::Five=>4,
            TileSprite::Six=>5,
            TileSprite::Seven=>6,
            TileSprite::Eight=>7,
            TileSprite::Nine=>8,
            TileSprite::Ten=>9,
            TileSprite::Eleven=>10,
            TileSprite::Twelve=>11,
            TileSprite::Thirteen=>12,
            TileSprite::Fourteen=>13,
            TileSprite::Fifteen=>14,
            TileSprite::Empty=>15
        }
    }
}


impl FromWorld for SpriteAtlas {
    fn from_world(world: &mut World) -> Self {
        let assets = world.get_resource::<AssetServer>().unwrap();
        let texture_handle = assets.load("sprite_atlas.png");

        let mut texture_atlases = world.get_resource_mut::<Assets<TextureAtlas>>().unwrap();

        let texture_atlas =
            TextureAtlas::from_grid(texture_handle, Vec2::new(4.0, 4.0), 4, 4, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        SpriteAtlas(texture_atlas_handle)
    }
}

fn add_sprite_to_indexable<T: Component + IndexableSprite>(
    mut commands: Commands,
    sprites: Query<Entity, (Added<T>, Without<TextureAtlasSprite>)>,
    atlas: Res<T::AtlasHandleWrapper>,
) {
    for character in &sprites {
        let handle = atlas.deref().deref();
        commands.entity(character).insert((
            handle.clone(),
            TextureAtlasSprite {
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        ));
    }
}

fn update_indexable_sprite<T: Component + IndexableSprite>(
    mut sprites: Query<(&T, &mut TextureAtlasSprite)>,
) {
    for (indexable, mut sprite) in sprites.iter_mut() {
        sprite.index = indexable.index();
    }
}

pub trait IndexableSprite {
    type AtlasHandleWrapper: Resource + std::ops::Deref<Target = Handle<TextureAtlas>>;

    fn index(&self) -> usize;
}
