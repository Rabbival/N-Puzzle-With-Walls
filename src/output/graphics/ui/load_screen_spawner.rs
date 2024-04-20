use crate::prelude::*;

const LAYOUT_MARGINS_RECT: UiRect = UiRect {
    top: Val::Px(50.0),
    right: Val::Px(30.0),
    bottom: Val::Px(50.0),
    left: Val::Px(30.0)
};

pub struct LoadScreenSpawnerPlugin;

impl Plugin for LoadScreenSpawnerPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_load_screen_arrows,
            )
        );
    }
}

fn spawn_load_screen_arrows(
    mut button_event_reader: EventReader<SpawnLoaderButtons>,
    mut commands: Commands,
) {
    for button_event in button_event_reader.read() {
        spawn_load_screen_arrow(
            button_event,
            JustifyContent::End,
            ">",
            &mut commands
        );
        spawn_load_screen_arrow(
            button_event,
            JustifyContent::Start,
            "<",
            &mut commands
        );


        //TODO: change to actual buttons when the time comes
        //currently here to demonstrate how one can load only when a layout is chosen
        commands
            .spawn((
                build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::Center,
                    Visibility::Hidden,
                    Some(FlexDirection::Column)
                ),
                simple_on_screen_tag(AppState::Loader),
            ))
            .with_children(|parent| {
                //first row
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Start,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    spawn_layout_entity(
                        parent,
                        &button_event
                    );
                    spawn_layout_entity(
                        parent,
                        &button_event
                    );
                });
                //second row
                parent.spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                }).with_children(|parent| {
                    spawn_layout_entity(
                        parent,
                        &button_event
                    );
                    spawn_layout_entity(
                        parent,
                        &button_event
                    );
                });
            });
    }
}

fn spawn_layout_entity(
    parent: &mut ChildBuilder,
    button_event: &SpawnLoaderButtons
    //TODO: action
){
    parent.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_items: AlignItems::Center,
            margin: LAYOUT_MARGINS_RECT,
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        let mut layout_entity = parent.spawn((
            ButtonBundle {
                style: button_event.board_props_button_style.clone(),
                background_color: super::NORMAL_BUTTON_COLOR.into(),
                ..default()
            },
            //TODO: put action here
        ));
        layout_entity.with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(
                    DomainBoard::default().to_string_for_button(),
                    button_event.tiny_text_style.clone(),
                ),
                ButtonText,
            ));
        });
        //TODO: .with_children for the layout itself
    });
}

fn spawn_load_screen_arrow(
    button_event: &SpawnLoaderButtons,
    content_side: JustifyContent,
    text_value: impl Into<String>,
    //TODO: action
    commands: &mut Commands
){
    let thin_button_style = &button_event.thin_button_style;
    let button_text_style = &button_event.button_text_style;

    commands
        .spawn((
            build_node_bundle_with_full_percentage_style(
                AlignItems::Center,
                content_side,
                Visibility::Hidden,
                None
            ),
            simple_on_screen_tag(AppState::Loader),
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::INDIGO.into(),
                ..default()
            })
                .with_children(|parent| {
                    let mut arrow_button_entity = parent.spawn((
                        ButtonBundle {
                            style: thin_button_style.clone(),
                            background_color: super::NORMAL_BUTTON_COLOR.into(),
                            ..default()
                        },
                        //TODO: put action here
                    ));
                    arrow_button_entity.with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                text_value,
                                button_text_style.clone(),
                            ),
                            ButtonText,
                        ));
                    });
                });
        });

}