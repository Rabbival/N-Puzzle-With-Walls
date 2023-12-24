use crate::{prelude::*, output::graphics::menu_graphics, costume_event::{ui_event, screen_changing_event}};
use std::mem;

pub struct MenuUiLogicPlugin;

impl Plugin for MenuUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    set_chosen_options_to_fit_current_props,
                    update_menu_ui_after_press_general,
                    update_wall_count_menu_ui,
                    set_applied_props
                    
                )
                .run_if(in_state(GameState::Menu))
            )
            ;
    }
}


fn set_chosen_options_to_fit_current_props(
    mut event_listener: EventReader<screen_changing_event::SetMenuElementsToFitCurrent>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<AppliedOptionTag>)
    >,
    mut currently_applied: Query<
        (Entity, &mut BackgroundColor) , 
        (With<AppliedOptionTag>, Without<SelectedOptionTag>)
    >,
    mut commands: Commands
){
    for _event in event_listener.read(){
        // remove from previously chosen and not applied
        for (
            chosen_not_applied, 
            mut not_applied_button_color, 
            _
        ) in currently_chosen.iter_mut(){
            menu_graphics::set_color_to_normal(&mut not_applied_button_color);
            commands.entity(chosen_not_applied).remove::<SelectedOptionTag>();
        }

        // put the chosen mark in the currently applied ones
        for (
            should_be_marked_chosen,
            mut should_be_marked_button_color, 
        ) in currently_applied.iter_mut(){
            menu_graphics::set_color_to_pressed(&mut should_be_marked_button_color);
            commands.entity(should_be_marked_chosen).insert(SelectedOptionTag);
        }
    }
}


/// for the planned board properties updates that don't require special treatment
fn update_menu_ui_after_press_general(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
    >,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        let menu_button_action = button_event.action;
        let button_action_discriminant;
        let pressed_button_entity = button_event.entity;
        match menu_button_action{
            MenuButtonAction::ChangeSize(_)
                | MenuButtonAction::ChangeEmptyTilesCount(_)
                | MenuButtonAction::ChangeGenerationMethod(_)
            => {
                button_action_discriminant=mem::discriminant(&menu_button_action);
            },
            _=> continue,
        }

        for (
            previous_button, 
            mut previous_color, 
            menu_button_action_of_chosen
        ) in currently_chosen.iter_mut(){
            if button_action_discriminant == mem::discriminant(menu_button_action_of_chosen){
                menu_graphics::set_color_to_normal(&mut previous_color);
                commands.entity(previous_button).remove::<SelectedOptionTag>();
                commands.entity(pressed_button_entity).insert(SelectedOptionTag);
            }  
        }
    }
}

fn update_wall_count_menu_ui(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
    >,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action{
            let menu_button_action = button_event.action;
            let pressed_button_entity = button_event.entity;
            let (apply_button, mut apply_button_color) = apply_button_query.single_mut();
            match wall_count_action{
                WallTilesChange::Apply=> {
                    commands.entity(button_event.entity).insert(SelectedOptionTag);

                    // take care of selected option tag
                    for (
                        previous_button, 
                        mut previous_color, 
                        menu_button_action_of_chosen
                    ) in currently_chosen.iter_mut(){
                        if mem::discriminant(&menu_button_action) == mem::discriminant(menu_button_action_of_chosen){
                            menu_graphics::set_color_to_normal(&mut previous_color);
                            commands.entity(previous_button).remove::<SelectedOptionTag>();
                            commands.entity(pressed_button_entity).insert(SelectedOptionTag);
                        }  
                    }
                },
                WallTilesChange::Increase | WallTilesChange::Decrease=> {
                    menu_graphics::set_color_to_normal(&mut apply_button_color);
                    commands.entity(apply_button).remove::<SelectedOptionTag>();
                }
            }
        }      
    }
}


fn set_applied_props(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut currently_chosen: Query<
        (Entity, &mut BackgroundColor, &MenuButtonAction), 
        (With<SelectedOptionTag>, Without<ApplyButtonTag>)
    >,
    mut currently_applied: Query<
        Entity,
        (With<AppliedOptionTag>, Without<SelectedOptionTag>)
    >,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::GenerateBoard = button_event.action{
            // remove applied from previous settings
            for previously_applied in currently_applied.iter_mut(){
                commands.entity(previously_applied).remove::<AppliedOptionTag>();
            }
            // insert applied to the new settings
            for (previous_button, _ , _ ) in currently_chosen.iter_mut(){
                commands.entity(previous_button).insert(AppliedOptionTag);
            }
        }
    }
}