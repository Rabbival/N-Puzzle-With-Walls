use crate::output::graphics::ui::UiGraphicsPlugin;
use crate::prelude::*;


pub mod tile_graphics;
pub mod camera;
pub mod ui;
pub mod visibility_tags;

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
            show_only_if_has_specified_screen_tag
                .in_set(StateChangeSystemSets::HandleStateChange),
        );
    }
}

fn show_only_if_has_specified_screen_tag(
    app_next_state: Res<State<AppState>>,
    mut toggle_their_visibility: Query<(
        &mut Visibility,
        Option<&OnOwnScreenVisibility>,
        &CustomOnScreenTag,
    )>,
) {
    if app_next_state.is_changed() {
        for (mut visibility, optional_own_screen_vis, entity_tag) in
            toggle_their_visibility.iter_mut()
        {
            if *app_next_state == entity_tag.0 {
                if let Some(own_screen_vis) = optional_own_screen_vis {
                    *visibility = own_screen_vis.0;
                }else{
                    *visibility = Visibility::Visible;
                }
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}