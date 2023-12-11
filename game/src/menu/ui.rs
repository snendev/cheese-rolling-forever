use bevy::prelude::*;

use crate::AppState;

const GAME_TITLE: &str = "Cheese";

#[derive(Component)]
pub(super) struct MenuUI;
#[derive(Component)]
pub(super) struct PlayButton;

// bg_color.0 = Color::OLIVE;
// bg_color.0 = Color::TEAL;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
pub(super) fn handle_play(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut state: ResMut<NextState<AppState>>,
) {
    for (interaction, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                bg_color.0 = PRESSED_BUTTON;
                state.set(AppState::SpawningScene);
            }
            Interaction::Hovered => {
                bg_color.0 = HOVERED_BUTTON;
            }
            _ => {
                bg_color.0 = NORMAL_BUTTON;
            }
        }
    }
}

pub(super) fn spawn_start_menu(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Menu UI"),
            MenuUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::percent(4., 4., 6., 6.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    Name::new("Left UI"),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(40.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceBetween,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((Name::new("Title"), title_node()));
                    builder
                        .spawn((Name::new("Play Button"), PlayButton, play_button()))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("Play Button Text"),
                                TextBundle::from_section(
                                    "Play",
                                    TextStyle {
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..Default::default()
                                    },
                                ),
                            ));
                        });
                });
            builder
                .spawn((
                    Name::new("Right UI"),
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(40.0),
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::FlexEnd,
                            // padding: UiRect::percent(0., 0., 0., 5.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .with_children(|builder| {
                    builder.spawn((Name::new("Instructions"), instructions()));
                });
        });
}

fn title_node() -> TextBundle {
    TextBundle::from_section(
        format!("{}", GAME_TITLE),
        TextStyle {
            font_size: 128.,
            color: Color::rgba(1., 1., 1., 0.75),
            ..Default::default()
        },
    )
}

fn instructions() -> TextBundle {
    let style = TextStyle {
        font_size: 32.,
        color: Color::rgba(1., 1., 1., 0.75),
        ..Default::default()
    };
    TextBundle::from_sections(vec![
        TextSection::new(
            "To steer/move left/right, press the Left and Right keys.\n",
            style.clone(),
        ),
        TextSection::new("Look backward with Space.", style),
    ])
}

fn play_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            margin: UiRect::percent(0., 0., 0., 5.),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..Default::default()
    }
}

fn _audio_slider() -> NodeBundle {
    NodeBundle {
        ..Default::default()
    }
}
