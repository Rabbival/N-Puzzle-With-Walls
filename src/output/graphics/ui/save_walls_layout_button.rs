use crate::prelude::*;

#[derive(Component)]
pub struct SaveWallsLayoutButton;

#[derive(Component)]
pub struct SaveWallsLayoutTextTag;

pub struct GameScreenButtonSpawnerPlugin;

impl Plugin for GameScreenButtonSpawnerPlugin {
    fn build(&self, app: &mut App) {
        
        app
            .add_systems(Startup, spawn_save_walls_layout_button)
            .add_systems(Update, flash_save_walls_layout_button_text_red);
    }
}

fn spawn_save_walls_layout_button(
    mut big_button_event_reader: EventReader<SpawnBigButtons>,
    mut commands: Commands,
) {
    for big_button_event in big_button_event_reader.read() {
        let button_style = &big_button_event.save_walls_layout_button_style;
        let button_text_style = &big_button_event.save_walls_layout_button_text_style;
        let tiny_red_text_style = &big_button_event.tiny_red_text_style;
        commands
            .spawn(
                (build_node_bundle_with_full_percentage_style(
                    AlignItems::Center,
                    JustifyContent::End,
                    Visibility::Hidden,
                    Some(FlexDirection::ColumnReverse)
                ),
                 simple_on_screen_tag(AppState::Game)
                ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: super::NORMAL_BUTTON_COLOR.into(),
                                    ..default()
                                },
                                SaveWallsLayoutButton,
                            ))
                            .with_children(|parent| {
                                parent.spawn((TextBundle::from_section(
                                    "Save Walls Layout",
                                    button_text_style.clone(),
                                ), 
                                    ButtonText,
                                    SaveWallsLayoutTextTag
                                ));
                            });
                    });
                parent
                    .spawn((
                        TextBundle::from_section(
                            "",
                            tiny_red_text_style.clone()
                        ),
                        TextAboveSaveButton
                    ));
            });
    }
}

fn flash_save_walls_layout_button_text_red(
    // mut event_listener: EventReader<BoardSavingError>,
    // mut save_walls_layout_text: Query<&mut Text, With<SaveWallsLayoutTextTag>>
){
    // for _ in event_listener.read(){
    //     let generation_text_color = &mut save_walls_layout_text.single_mut().sections[0].style.color;
    //     *generation_text_color = super::RED_TEXT_COLOR;
    // }
}