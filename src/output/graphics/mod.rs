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
    mut single_screen_entities: Query<(
        &mut Visibility,
        Option<&OnOwnScreenVisibility>,
        &CustomOnScreenTag,
    ), Without<MultipleOnScreenTags>>,
    mut multiple_screen_entities: Query<(&mut Visibility, &MultipleOnScreenTags), Without<CustomOnScreenTag>>,
) {
    if app_next_state.is_changed() {
        for (mut visibility, optional_own_screen_vis, entity_tag) in
            single_screen_entities.iter_mut()
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
        'entity_for: for (mut visibility, screen_tags) in
        multiple_screen_entities.iter_mut()
        {
            for screen_tag in screen_tags.0.iter() {
                if *app_next_state == screen_tag.0 {
                    *visibility = Visibility::Visible;
                    continue 'entity_for;
                }
            }
            *visibility = Visibility::Hidden;
        }
    }
}