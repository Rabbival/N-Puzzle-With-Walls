use crate::output::graphics::ui::RED_TEXT_COLOR;
use crate::prelude::*;

pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    (update_wall_tiles_count_visuals)
                        .run_if(resource_changed::<UnappliedMenuWallCount>()),
                )
                    .run_if(in_state(AppState::Menu)),
            ),
        );
    }
}

fn update_wall_tiles_count_visuals(
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>,
    mut wall_count_text_query: Query<&mut Text, With<WallCountTextTag>>,
) {
    let mut text = wall_count_text_query.single_mut();
    text.sections[0].value = unapplied_menu_wall_count.0.to_string();
}