use crate::output::graphics::ui::NORMAL_BUTTON_COLOR;
use crate::prelude::*;

#[derive(Component)]
pub struct PopUpMessageTextTag;

#[derive(Component)]
pub struct PopUpMessageDynamicTextTag;

#[derive(Component)]
pub struct TextAbovePopUpMessageButtons;

pub struct PopUpMessagePlugin;

impl Plugin for PopUpMessagePlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Startup,
                spawn_pop_up_message
            )
            .add_systems(
                Update,(
                    set_pop_up_dynamic_text_box_color,
                ));
    }
}

pub fn set_are_you_sure_message_type_and_text(
    requested_new_type: PopUpMessageType,
    text_ref: &mut String,
    type_ref: &mut PopUpMessageType
) {
    *text_ref = requested_new_type.to_string();
    *type_ref = requested_new_type;
}

fn set_pop_up_dynamic_text_box_color(
    mut reset_text_event_reader: EventReader<AllowPlayerToSetBoardName>,
    mut set_text_event_reader: EventReader<UpdateNewbornDomainBoardName>,
    mut pop_up_dynamic_text_entity_query: Query<&mut BackgroundColor, With<PopUpMessageDynamicTextTag>>,
){
    for _reset_request in reset_text_event_reader.read(){
        pop_up_dynamic_text_entity_query.single_mut().0 = Color::DARK_GRAY;
    }
    for _set_request in set_text_event_reader.read(){
        pop_up_dynamic_text_entity_query.single_mut().0 = Color::NONE;
    }
}

fn spawn_pop_up_message(
    mut spawn_event_reader: EventReader<SpawnTextsAndButtons>,
    mut commands: Commands,
) {
    for spawn_request in spawn_event_reader.read() {
        let text_style = &spawn_request.medium_text_style;
        let button_style = &spawn_request.big_button_style;
        let tiny_red_text_style = &spawn_request.tiny_red_text_style;
        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Center,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                PopUpMessageType::DeleteAllBoards,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(85.0),
                            height: Val::Percent(50.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        z_index: ZIndex::Global(1),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(95.0),
                                    height: Val::Percent(95.0),
                                    flex_direction: FlexDirection::Column,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..default()
                                },
                                background_color: Color::BLACK.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                        PopUpMessageType::DeleteAllBoards.to_string(),
                                        text_style.clone(),
                                    ).with_text_justify(JustifyText::Center),
                                  ButtonText,
                                  PopUpMessageTextTag
                                ));
                                parent.spawn(pop_up_message_ui_gap());
                                parent.spawn((
                                    TextBundle::from_section(
                                        String::default(),
                                        text_style.clone(),
                                    ).with_text_justify(JustifyText::Center)
                                        .with_background_color(Color::DARK_GRAY.into(),),
                                  ButtonText,
                                  PopUpMessageDynamicTextTag
                                ));
                                parent.spawn(pop_up_message_ui_gap());
                                parent.spawn((
                                    TextBundle::from_section(
                                        "",
                                        tiny_red_text_style.clone()
                                    ),
                                    TextAbovePopUpMessageButtons
                                ));
                                parent.spawn(pop_up_message_ui_gap());
                                parent.spawn(NodeBundle {
                                    style: Style {
                                        width: Val::Percent(95.0),
                                        height: Val::Percent(30.0),
                                        flex_direction: FlexDirection::Row,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    background_color: Color::INDIGO.into(),
                                    ..default()
                                }).with_children(|parent| {
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: button_style.clone(),
                                                background_color: NORMAL_BUTTON_COLOR.into(),
                                                ..default()
                                            },
                                            PopUpMessageButtonAction::Confirm
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Confirm",
                                                text_style.clone(),
                                            ));
                                        });
                                    parent.spawn(pop_up_message_ui_gap());
                                    parent
                                        .spawn((
                                            ButtonBundle {
                                                style: button_style.clone(),
                                                background_color: NORMAL_BUTTON_COLOR.into(),
                                                ..default()
                                            },
                                            PopUpMessageButtonAction::Cancel
                                        ))
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle::from_section(
                                                "Cancel",
                                                text_style.clone(),
                                            ));
                                        });
                                });
                            });
                        });
                parent.spawn(pop_up_message_ui_gap());
            });
    }
}

fn pop_up_message_ui_gap() -> NodeBundle{
    NodeBundle {
        style: Style {
            width: Val::Percent(40.0),
            height: Val::Percent(20.0),
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    }
}