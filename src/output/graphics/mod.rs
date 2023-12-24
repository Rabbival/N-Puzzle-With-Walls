use crate::prelude::*;

pub mod tile_graphics;
pub mod menu_graphics;
pub mod menu_spawner;


pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                TileGraphicsPlugin,
                MenuGraphicsPlugin,
                MenuSpanwerPlugin
            ))
            .add_systems(Update, (
                    despawn_entities_with_tag,
                    toggle_visibility_for_entities_with_tag
                )
                .chain()
                .in_set(StateChangeSystemSets::HandleStateChange)
            )
            ;
    }
}

/// despawns all entities with that tag
fn despawn_entities_with_tag(
    mut event_listener: EventReader<DespawnElementsWithTag>,
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


/// hides if visible, shows if hidden- all entities with that tag
fn toggle_visibility_for_entities_with_tag(
    mut event_listener: EventReader<ToggleVisibilityForElementsWithTag>,
    mut to_despawn: Query<(&mut Visibility, &OnScreenTag)>, 
) {
    for tag_container in event_listener.read(){
        for (mut visibility, entity_tag) in to_despawn.iter_mut() {
            if tag_container.0 == *entity_tag{
                if *visibility == Visibility::Hidden {
                    *visibility = Visibility::Visible;
                } else if *visibility == Visibility::Visible{
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}