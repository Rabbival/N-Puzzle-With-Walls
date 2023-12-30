use crate::{prelude::*, output::graphics::menu_graphics, costume_event::{ui_event, screen_changing_event}};
use std::mem;

pub struct MenuUiLogicPlugin;

impl Plugin for MenuUiLogicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                    set_chosen_options_to_fit_current_props.in_set(StateChangeSystemSets::HandleStateChange),
                    (
                        update_menu_ui_after_press_general,
                        increase_or_decrease_wall_count_menu_ui_update,
                        set_applied_props
                    ).in_set(InputSystemSets::InputHandling),
                    apply_wall_count_menu_ui_update.in_set(InputSystemSets::PostMainChanges),
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

fn increase_or_decrease_wall_count_menu_ui_update(
    mut button_event_listener: EventReader<ui_event::ButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut commands: Commands,
){
    for button_event in button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action{
            let (apply_button, mut apply_button_color) = apply_button_query.single_mut();
            match wall_count_action{
                WallTilesChange::Increase | WallTilesChange::Decrease=> {
                    menu_graphics::set_color_to_normal(&mut apply_button_color);
                    commands.entity(apply_button).remove::<SelectedOptionTag>();
                },
                _ => {}
            }
        }      
    }
}

fn apply_wall_count_menu_ui_update(
    mut apply_button_event_listener: EventReader<ui_event::ApplyButtonPressed>,
    mut apply_button_query: Query<(Entity, &mut BackgroundColor), With<ApplyButtonTag>>,
    mut commands: Commands,
){
    for button_event in apply_button_event_listener.read(){
        if let MenuButtonAction::ChangeWallTilesCount(wall_count_action) = button_event.action{
            if let WallTilesChange::Apply = wall_count_action{
                let (apply_button_entity, mut apply_button_color) = apply_button_query.single_mut();
                commands.entity(apply_button_entity).insert(SelectedOptionTag);
                menu_graphics::set_color_to_pressed(&mut apply_button_color);
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