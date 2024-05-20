use enum_iterator::all;
use crate::prelude::*;

#[derive(Component, Default)]
pub struct TileDictionary {
    pub entity_by_tile: HashMap<Tile, Option<Entity>>,
}

pub struct TileDictionaryPlugin;

impl Plugin for TileDictionaryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_tile_dictionary);
    }
}

fn spawn_tile_dictionary(mut commands: Commands) {
    commands.spawn(TileDictionary::default());
    for loader_screen_slot in all::<LoaderScreenSlot>(){
        commands.spawn((
           loader_screen_slot,
           TileDictionary::default()
        ));
    }
}
