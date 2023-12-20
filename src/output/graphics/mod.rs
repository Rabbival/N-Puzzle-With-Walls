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

/// hides all entities with component specified in ::<>
fn despawn_entities_with_tag(
    mut event_listener: EventReader<HideElementsWithTag>,
    to_despawn: Query<(Entity, &OnScreenTag)>, 
    mut commands: Commands
) {
    for tag_container in event_listener.read(){
        for (entity, entity_tag) in to_despawn.iter() {
            if tag_container.0 == *entity_tag{
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}


/// despawns all entities with component specified in ::<>
fn hide_entities_with_tag(
    mut event_listener: EventReader<DespawnElementsWithTag>,
    mut to_despawn: Query<(&mut Visibility, &OnScreenTag)>, 
) {
    for tag_container in event_listener.read(){
        for (mut visibility, entity_tag) in to_despawn.iter_mut() {
            if tag_container.0 == *entity_tag{
                *visibility = Visibility::Hidden;
            }
        }
    }
}
