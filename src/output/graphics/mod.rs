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
            .add_systems(Update, 
                toggle_visibility_for_entities_with_tag
                    .in_set(StateChangeSystemSets::HandleStateChange)
            )
            ;
    }
}


/// hides if visible, shows if hidden- all entities with that tag
fn toggle_visibility_for_entities_with_tag(
    mut event_listener: EventReader<ToggleVisibilityForElementsWithTag>,
    mut toggle_their_visibility: Query<(&mut Visibility, &OnScreenTag)>, 
) {
    for tag_container in event_listener.read(){
        for (mut visibility, entity_tag) in toggle_their_visibility.iter_mut() {
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