use crate::prelude::*;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

/// Which option is currently selected
#[derive(Component)]
struct SelectedOptionTag;


pub struct UiGraphicsPlugin;

impl Plugin for UiGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            
            ;
    }
}