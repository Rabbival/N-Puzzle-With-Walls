use crate::{prelude::*, costume_event::reset_event};

const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

/// Which option is currently selected
#[derive(Component)]
struct SelectedOptionTag;

#[derive(Component)]
struct MenuButtonAction(BoardSize);


pub struct UiGraphicsPlugin;

impl Plugin for UiGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, settings_menu_setup)
            .add_systems(
                Update,(
                update_button_color,
                menu_action
                ).run_if(in_state(GameState::Menu)),
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

fn menu_action(
    mut input_event_writer: EventWriter<reset_event::ResetBoardLogic>,
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut game_state: ResMut<NextState<GameState>>,
    mut board_size_res: ResMut<BoardSize>
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            *board_size_res = menu_button_action.0;
            input_event_writer.send(reset_event::ResetBoardLogic{reroll_solved: true});

            game_log(GameLog::BoardSizeChanged(menu_button_action.0));
            game_state.set(GameState::Game);
        }
    }
}

fn settings_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0 ,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            OnScreenTag::Menu,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::INDIGO.into(),
                    ..default()
                })
                .with_children(|parent| {
                    for board_size in BoardSize::as_list(){
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction(board_size)
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    board_size.to_string(),
                                    button_text_style.clone(),
                                ));
                            });
                    }
             });
        });
}