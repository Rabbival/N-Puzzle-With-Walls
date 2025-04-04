use crate::prelude::*;
use enum_iterator::all;

#[derive(Component, Default)]
pub struct TileDictionary {
    pub entity_by_tile: HashMap<Tile, Option<Entity>>,
}

impl TileDictionary {
    pub fn extract_tile_entity(&self, tile: &Tile) -> Result<Entity, EntityRelatedCustomError> {
        match self.entity_by_tile.get(tile) {
            None => Err(EntityRelatedCustomError::DataStructError(
                DataStructError::ItemNotFound(*tile),
            )),
            Some(optional_entity) => match optional_entity {
                None => Err(EntityRelatedCustomError::NoEntity),
                Some(entity) => Ok(*entity),
            },
        }
    }
}

pub struct TileDictionaryPlugin;

impl Plugin for TileDictionaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_tile_dictionary);
    }
}

fn spawn_tile_dictionary(mut commands: Commands) {
    commands.spawn(TileDictionary::default());
    for loader_screen_slot in all::<LoaderScreenSlot>() {
        commands.spawn((loader_screen_slot, TileDictionary::default()));
    }
}
