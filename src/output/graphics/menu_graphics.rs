use crate::{prelude::*, costume_event::{reset_event, ui_event}};

const NORMAL_BUTTON: Color = Color::rgb(0.1, 0.1, 0.1);
const HOVERED_BUTTON: Color = Color::rgb(0.2, 0.2, 0.2);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);
const PRESSED_BUTTON: Color = Color::rgb(0.3, 0.3, 0.3);

/// Which option is currently selected
#[derive(Component)]
struct SelectedOptionTag;

#[derive(Component, Debug)]
pub enum MenuButtonAction{
    ChangeSize(BoardSize),
    ChangeWallTilesCount(u8),
    ChangeEmptyCount(u8),
    ChangeGenerationMethod,
    GenerateBoard
}


pub struct MenuGraphicsPlugin;

impl Plugin for MenuGraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, settings_menu_setup)
            .add_systems(Startup, (
                    spawn_generate_button,
                    spawn_size_options,
                )
                .after(settings_menu_setup)
            )
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
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Pressed {
            match menu_button_action{
                MenuButtonAction::ChangeSize(board_size)=> {
                    *board_size_res = *board_size;
                },
                MenuButtonAction::ChangeWallTilesCount(walls_count)=> {

                },
                MenuButtonAction::ChangeEmptyCount(empty_count)=> {

                },
                MenuButtonAction::ChangeGenerationMethod=> {

                },
                MenuButtonAction::GenerateBoard=>{
                    input_event_writer.send(reset_event::ResetBoardLogic{reroll_solved: true});
                    game_state.set(GameState::Game);
                }
            };

            game_log(GameLog::BoardSettingsChanged(menu_button_action));
        }
    }
}

fn settings_menu_setup(
    mut button_event_writer: EventWriter<ui_event::SpawnButtons>,
    mut big_button_event_writer: EventWriter<ui_event::SpawnBigButtons>
) {
    let button_style = Style {
        width: Val::Px(150.0),
        height: Val::Px(50.0),
        margin: UiRect::all(Val::Px(15.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let big_button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(80.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let big_button_text_style = TextStyle {
        font_size: 60.0 ,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0 ,
        ..default()
    };

    button_event_writer.send(ui_event::SpawnButtons{
        button_style,
        button_text_style
    });
    big_button_event_writer.send(ui_event::SpawnBigButtons{
        big_button_style,
        big_button_text_style
    });
}

fn spawn_generate_button(
    mut big_button_event_reader: EventReader<ui_event::SpawnBigButtons>,
    mut commands: Commands
){
    for big_button_event in big_button_event_reader.read(){
        let button_style=&big_button_event.big_button_style;
        let button_text_style=&big_button_event.big_button_text_style;

        commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::End,
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
                .spawn((
                    ButtonBundle {
                        style: button_style.clone(),
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    MenuButtonAction::GenerateBoard
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Generate",
                        button_text_style.clone(),
                    ));
                });
            });
    }
}

fn spawn_size_options(
    mut button_event_reader: EventReader<ui_event::SpawnButtons>,
    mut commands: Commands
){
    for button_event in button_event_reader.read(){
        let button_style=&button_event.button_style;
        let button_text_style=&button_event.button_text_style;

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
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
                        //title
                        parent.spawn(TextBundle::from_section(
                            String::from("Board Sizes"),
                            button_text_style.clone(),
                        ));
                        //buttons
                        for board_size in BoardSize::as_list(){
                            let mut button_entity = parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    MenuButtonAction::ChangeSize(board_size)
                                ));    
                                button_entity.with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        board_size.to_string(),
                                        button_text_style.clone(),
                                    ));
                                });
                                if board_size == BoardSize::default() {
                                    button_entity.insert(SelectedOptionTag);
                                }
                        }
                });
            });
    }
}