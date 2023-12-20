use crate::prelude::*;

pub mod tile_graphics;
pub mod ui_graphics;


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TileGraphicsPlugin,
                UiGraphicsPlugin,
            ))
            
            ;
    }
}

/// despawns all entities with component specified in ::<>
fn despawn_entities_with_tag(
    mut event_listener: EventReader<DespawnElementsTaggedWith>,
    to_despawn: Query<Entity, With<OnScreenTag>>, 
    mut commands: Commands
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
