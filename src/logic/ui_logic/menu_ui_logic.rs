use crate::prelude::*;
use std::mem;

pub struct MenuUiLogicPlugin;

impl Plugin for MenuUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_chosen_options_to_fit_current_props
                    .in_set(StateChangeSystemSets::HandleStateChange),
                (
                    update_menu_ui_after_press_general,
                    increase_or_decrease_wall_count_menu_ui_update,
                    set_applied_props,
                    set_tree_generation_options_visibility,
                )
                    .in_set(InputSystemSets::InputHandling),
                apply_wall_count_menu_ui_update.in_set(InputSystemSets::PostMainChanges),
            )
                .run_if(in_state(AppState::Menu)),
        );
    }
}

fn set_chosen_options_to_fit_current_props(
    mut event_listener: EventReader<SetMenuElementsToFitCurrent>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction),
        (With<SelectedOptionTag>, Without<AppliedOptionTag>),
    >,
    mut currently_applied: Query<
        (Entity, &mut BackgroundColor),
        (With<AppliedOptionTag>, Without<SelectedOptionTag>),
    >,
    mut commands: Commands,
) {
    for _event in event_listener.read() {
        // remove from previously chosen and not applied
        for (chosen_not_applied, mut not_applied_button_color, _) in currently_chosen.iter_mut() {
            set_color_to_normal(&mut not_applied_button_color);
            commands
                .entity(chosen_not_applied)
                .remove::<SelectedOptionTag>();
        }

        // put the chosen mark in the currently applied ones
        for (should_be_marked_chosen, mut should_be_marked_button_color) in
            currently_applied.iter_mut()
        {
            set_color_to_pressed(&mut should_be_marked_button_color);
            commands
                .entity(should_be_marked_chosen)
                .insert(SelectedOptionTag);
        }
    }
}

/// for the planned board properties updates that don't require special treatment
fn update_menu_ui_after_press_general(
    mut button_event_listener: EventReader<MenuButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction),
        (With<SelectedOptionTag>, Without<ApplyButtonTag>),
    >,
    mut commands: Commands,
) {
    for button_event in button_event_listener.read() {
        let menu_button_action = button_event.action;
        let pressed_button_entity = button_event.entity;
        let button_action_discriminant = match menu_button_action {
            MenuButtonAction::ChangeSize(_)
            | MenuButtonAction::ChangeEmptyTilesCount(_)
            | MenuButtonAction::ChangeGenerationMethod(_)
            | MenuButtonAction::ChangeSpanningTreeGeneration(_) => {
                mem::discriminant(&menu_button_action)
            }
            _ => continue,
        };

        for (previous_button, mut previous_color, menu_button_action_of_chosen) in
            currently_chosen.iter_mut()
        {
            if button_action_discriminant == mem::discriminant(menu_button_action_of_chosen) {
                set_color_to_normal(&mut previous_color);
                commands
                    .entity(previous_button)
                    .remove::<SelectedOptionTag>();
                commands
                    .entity(pressed_button_entity)
                    .insert(SelectedOptionTag);
            }
        }
    }
}

fn increase_or_decrease_wall_count_menu_ui_update(
    mut button_event_listener: EventReader<MenuButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut commands: Commands,
) {
    for button_event in button_event_listener.read() {
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action {
            let (apply_button, mut apply_button_color) = apply_button_query.single_mut();
            match wall_count_action {
                WallTilesChange::Increase | WallTilesChange::Decrease => {
                    set_color_to_normal(&mut apply_button_color);
                    commands.entity(apply_button).remove::<SelectedOptionTag>();
                }
                _ => {}
            }
        }
    }
}

fn apply_wall_count_menu_ui_update(
    mut apply_button_event_listener: EventReader<ApplyButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut commands: Commands,
) {
    for button_event in apply_button_event_listener.read() {
        if let MenuButtonAction::ChangeWallTilesCount(WallTilesChange::Apply) = button_event.action
        {
            let (apply_button_entity, mut apply_button_color) = apply_button_query.single_mut();
            commands
                .entity(apply_button_entity)
                .insert(SelectedOptionTag);
            set_color_to_pressed(&mut apply_button_color);
        }
    }
}

fn set_tree_generation_options_visibility(
    unapplied_menu_wall_count: Res<UnappliedMenuWallCount>,
    mut tree_generation_options_query: Query<
        (&mut Visibility, &mut OnOwnScreenVisibility),
        With<TreeGenerationOptionsTag>,
    >,
) {
    if unapplied_menu_wall_count.is_changed() {
        let (mut current_visibility, mut own_screen_vis_for_toggle) =
            tree_generation_options_query.single_mut();
        if unapplied_menu_wall_count.0 == 0 {
            *current_visibility = Visibility::Hidden;
            own_screen_vis_for_toggle.0 = Visibility::Hidden;
        } else {
            *current_visibility = Visibility::Visible;
            own_screen_vis_for_toggle.0 = Visibility::Visible;
        }
    }
}

fn set_applied_props(
    mut button_event_listener: EventReader<MenuButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction),
        (With<SelectedOptionTag>, Without<ApplyButtonTag>),
    >,
    mut currently_applied: Query<Entity, (With<AppliedOptionTag>, Without<SelectedOptionTag>)>,
    mut commands: Commands,
) {
    for button_event in button_event_listener.read() {
        if let MenuButtonAction::GenerateBoard = button_event.action {
            // remove applied from previous settings
            for previously_applied in currently_applied.iter_mut() {
                commands
                    .entity(previously_applied)
                    .remove::<AppliedOptionTag>();
            }
            // insert applied to the new settings
            for (previous_button, _, _) in currently_chosen.iter_mut() {
                commands.entity(previous_button).insert(AppliedOptionTag);
            }
        }
    }
}
