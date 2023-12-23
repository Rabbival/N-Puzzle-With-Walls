use crate::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
pub const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);


pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(
                    (update_button_color).run_if(in_state(GameState::Menu)),
                    (update_wall_tiles_count_visuals).run_if(resource_changed::<UnappliedMenuWallCount>())
                ))
            ;
    }
}


fn update_button_color(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOptionTag>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in &mut interaction_query {
        *color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}

fn update_wall_tiles_count_visuals(
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>,
    mut wall_count_text_query: Query<&mut Text , With<WallCountTextTag>>
){
    let mut text = wall_count_text_query.single_mut();
    text.sections[0].value = unapplied_menu_wall_count.0.to_string();
}


pub fn set_color_to_normal(background_color: &mut BackgroundColor){
    *background_color = NORMAL_BUTTON.into();
}