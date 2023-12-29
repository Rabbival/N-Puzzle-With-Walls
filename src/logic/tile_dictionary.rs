use crate::prelude::*;

#[derive(Component)]
pub struct TileDictionaryTag;

#[derive(Component)]
pub struct TileDictionary{
    pub entity_by_tile_type: HashMap<IndexedValue<TileType>,Option<Entity>>
}

pub struct TileDictionaryPlugin;

impl Plugin for TileDictionaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_tile_dictionary);
    }
}

fn spawn_tile_dictionary(mut commands: Commands){
    commands.spawn((
        TileDictionary{
            entity_by_tile_type: HashMap::<IndexedValue<TileType>,Option::<Entity>>::new()
        },
        TileDictionaryTag
    ));
}

