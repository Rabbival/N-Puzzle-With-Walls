use crate::prelude::*;
use std::mem;

pub struct MenuUiLogicPlugin;

impl Plugin for MenuUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::Menu),
                show_options_that_hide_when_loading_if_not_loading
            )
            .add_systems(
            Update,
            ((
                update_wall_tiles_count_visuals
                    .run_if(resource_changed::<UnappliedMenuWallCount>),
                (
                    update_menu_ui_after_press_general,
                    increase_or_decrease_wall_count_menu_ui_update,
                    show_applied_props,
                    set_tree_generation_options_visibility,
                    toggle_options_relevant_to_loader
                )
                    .in_set(InputSystemSets::InputHandling),
                apply_wall_count_menu_ui_update.in_set(InputSystemSets::PostMainChanges),
            )
                .run_if(in_state(AppState::Menu)),
             (
                 listen_for_applied_tag_change_requests,
                 set_chosen_options_to_fit_current_props
             ).chain()
                .in_set(StateChangeSystemSets::HandleStateChange),
            )
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

fn listen_for_applied_tag_change_requests(
    mut event_reader: EventReader<SetAppliedTagForProperty>,
    selected_options_query: Query<(Entity, &MenuButtonAction), With<AppliedOptionTag>>,
    not_selected_options_query: Query<(Entity, &MenuButtonAction), Without<AppliedOptionTag>>,
    mut commands: Commands
){
    for applied_tag_set_event in event_reader.read(){
        let variant_to_select = applied_tag_set_event.give_tag_to_variant;
        let discriminant_to_select = mem::discriminant(&variant_to_select);
        for (menu_button_entity, button_action) in &selected_options_query{
            if mem::discriminant(button_action) == discriminant_to_select{
                commands
                    .entity(menu_button_entity)
                    .remove::<AppliedOptionTag>();
            }
        }
        for (menu_button_entity, button_action) in &not_selected_options_query{
            if variant_to_select == *button_action{
                commands.entity(menu_button_entity).insert(AppliedOptionTag);
            }
        }
    }
}

fn set_chosen_options_to_fit_current_props(
    mut event_reader: EventReader<SetMenuElementsToFitCurrent>,
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
    for _event in event_reader.read() {
        // remove from previously chosen and not applied
        for (chosen_not_applied, mut not_applied_button_color, _) in &mut currently_chosen {
            set_color_to_normal(&mut not_applied_button_color);
            commands
                .entity(chosen_not_applied)
                .remove::<SelectedOptionTag>();
        }

        // put the chosen mark in the currently applied ones
        for (should_be_marked_chosen, mut should_be_marked_button_color) in
            &mut currently_applied
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
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction),
        (With<SelectedOptionTag>, Without<ApplyButtonTag>),
    >,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
        let menu_button_action = button_event.action;
        let pressed_button_color_entity = button_event.entity;
        let button_action_discriminant = match menu_button_action {
            MenuButtonAction::ChangeSize(_)
            | MenuButtonAction::ChangeEmptyTilesCount(_)
            | MenuButtonAction::ChangeGenerationMethod(_)
            | MenuButtonAction::ChangeSpanningTreeGeneration(_) 
            | MenuButtonAction::ChangeBoardDifficulty(_) => {
                mem::discriminant(&menu_button_action)
            }
            _ => continue,
        };

        for (previous_button, mut previous_color, menu_button_action_of_chosen) in
            &mut currently_chosen
        {
            if button_action_discriminant == mem::discriminant(menu_button_action_of_chosen) {
                set_color_to_normal(&mut previous_color);
                commands
                    .entity(previous_button)
                    .remove::<SelectedOptionTag>();
                commands
                    .entity(pressed_button_color_entity)
                    .insert(SelectedOptionTag);
            }
        }
    }
}

fn toggle_options_relevant_to_loader(
    mut visibility_change_event_writer: EventWriter<SetEntityVisibility>,
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut menu_nodes: Query<(Entity, &mut CustomOnScreenTag, &mut HideByChosenGenerationMethod)>,
){
    for button_event in button_event_reader.read() {
        if let MenuButtonAction::ChangeGenerationMethod(new_generation_method) =
            button_event.action
        {
            let loader_is_chosen = new_generation_method == BoardGenerationMethod::Load;

            set_visibility_for_buttons_that_dont_appear_when_load_is_chosen(
                loader_is_chosen,
                &mut visibility_change_event_writer,
                &mut menu_nodes
            );
        }
    }
}

fn show_options_that_hide_when_loading_if_not_loading(
    applied_board_properties_query: Query<&BoardProperties, With<AppliedBoardProperties>>,
    mut visibility_change_event_writer: EventWriter<SetEntityVisibility>,
    mut menu_nodes: Query<(Entity, &mut CustomOnScreenTag, &mut HideByChosenGenerationMethod)>,
){
    let applied_board_properties = applied_board_properties_query.single();
    if applied_board_properties.generation_method != BoardGenerationMethod::Load{
        set_visibility_for_buttons_that_dont_appear_when_load_is_chosen(
            false,
            &mut visibility_change_event_writer,
            &mut menu_nodes
        );
    }
}

fn set_visibility_for_buttons_that_dont_appear_when_load_is_chosen(
    should_hide_elements_in_question: bool,
    visibility_change_event_writer: &mut EventWriter<SetEntityVisibility>,
    menu_nodes: &mut Query<(Entity, &mut CustomOnScreenTag, &mut HideByChosenGenerationMethod)>,
){
    for (
        node_entity, 
        mut on_screen_tag, 
        mut hide_when_choosing_gen_method
    ) in menu_nodes 
    {
        let gen_methods_to_hide_in =
            hide_when_choosing_gen_method.generation_methods_when_should_hide.clone();
        for gen_method in gen_methods_to_hide_in{
            if gen_method == BoardGenerationMethod::Load {
                let optional_new_visibility;
                if should_hide_elements_in_question {
                    hide_when_choosing_gen_method.visibility_otherwise =
                        on_screen_tag.on_own_screen_visibility;
                    optional_new_visibility = Some(Visibility::Hidden);
                }else if let Some(previous_visibility) = hide_when_choosing_gen_method.visibility_otherwise {
                    optional_new_visibility = Some(previous_visibility);
                    hide_when_choosing_gen_method.visibility_otherwise = None;
                }else{
                    optional_new_visibility = None;
                }
                if let Some(new_visibility) = optional_new_visibility{
                    on_screen_tag.on_own_screen_visibility = Some(new_visibility);
                    visibility_change_event_writer.send(SetEntityVisibility {
                        entity: node_entity,
                        visibility: new_visibility
                    });
                }
                break;
            }
        }
    }
}

fn increase_or_decrease_wall_count_menu_ui_update(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
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
    mut apply_button_event_reader: EventReader<ApplyButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut commands: Commands,
) {
    for button_event in apply_button_event_reader.read() {
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
        (&mut Visibility, &mut CustomOnScreenTag),
        With<TreeGenerationOptionsTag>,
    >,
) {
    if unapplied_menu_wall_count.is_changed() {
        let (mut current_visibility, mut custom_on_screen_tag) =
            tree_generation_options_query.single_mut();
        if let Some(own_screen_vis_for_toggle) = &mut custom_on_screen_tag.on_own_screen_visibility{
            if unapplied_menu_wall_count.0 == 0 {
                *current_visibility = Visibility::Hidden;
                *own_screen_vis_for_toggle = Visibility::Hidden;
            } else {
                *current_visibility = Visibility::Visible;
                *own_screen_vis_for_toggle = Visibility::Visible;
            }
        }
    }
}

fn show_applied_props(
    mut button_event_reader: EventReader<MenuButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction),
        (With<SelectedOptionTag>, Without<ApplyButtonTag>),
    >,
    mut currently_applied: Query<Entity, (With<AppliedOptionTag>, Without<SelectedOptionTag>)>,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
        if let MenuButtonAction::MainButtonPressed = button_event.action {
            // remove applied from previous settings
            for previously_applied in &mut currently_applied {
                commands
                    .entity(previously_applied)
                    .remove::<AppliedOptionTag>();
            }
            // insert applied to the new settings
            for (previous_button, _, _) in &mut currently_chosen {
                commands.entity(previous_button).insert(AppliedOptionTag);
            }
        }
    }
}
