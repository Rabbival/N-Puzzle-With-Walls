use crate::output::graphics::ui::UiGraphicsPlugin;
use crate::prelude::*;


pub mod tile_graphics;
pub mod camera;
pub mod ui;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiGraphicsPlugin,
            CameraPlugin,
            TileGraphicsPlugin,
        ))
        .add_systems(
            Update,
            toggle_visibility_for_entities_with_tag
                .in_set(StateChangeSystemSets::HandleStateChange),
        );
    }
}

/// allows optionally visible components to save their original visibility
#[derive(Component, Default)]
pub struct OnOwnScreenVisibility(pub Visibility);

/// hides if visible, shows if hidden- all entities with that tag
fn toggle_visibility_for_entities_with_tag(
    mut event_listener: EventReader<ToggleVisibilityForElementsWithTag>,
    mut toggle_their_visibility: Query<(
        &mut Visibility,
        Option<&OnOwnScreenVisibility>,
        &CustomOnScreenTag,
    )>,
) {
    for tag_container in event_listener.read() {
        for (mut visibility, optional_own_screen_vis, entity_tag) in
            toggle_their_visibility.iter_mut()
        {
            if tag_container.0 == *entity_tag {
                if *visibility == Visibility::Hidden {
                    if let Some(own_screen_vis) = optional_own_screen_vis {
                        *visibility = own_screen_vis.0;
                    } else {
                        *visibility = Visibility::Visible;
                    }
                } else if *visibility == Visibility::Visible {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}