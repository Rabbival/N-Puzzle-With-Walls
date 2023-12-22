use crate::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
pub const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
pub const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
pub const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

/// Which option is currently selected
#[derive(Component)]
pub struct SelectedOptionTag;

#[derive(Component)]
pub struct ApplyButtonTag;

#[derive(Component, Debug)]
pub enum MenuButtonAction{
    ChangeSize(BoardSize),
    ChangeWallTilesCount(WallTilesChange),
    ChangeEmptyTilesCount(u8),
    ChangeGenerationMethod(BoardGenerationMethod),
    GenerateBoard
}

#[derive(Debug)]
pub enum WallTilesChange{
    Increase,
    Decrease,
    Apply
}


pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update,(update_button_color).run_if(in_state(GameState::Menu)),
            );
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

pub fn set_color_to_normal(background_color: &mut BackgroundColor){
    *background_color = NORMAL_BUTTON.into();
}