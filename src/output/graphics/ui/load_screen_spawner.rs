use crate::prelude::*;

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
    }
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