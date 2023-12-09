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
                state.set(AppState::Starting);
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
                    width: Val::Percent(60.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((Name::new("Title Container"), title_container()))
                .with_children(|builder| {
                    builder.spawn((Name::new("Title"), title_node()));
                });
            builder
                .spawn((Name::new("Controls panel"), controls_panel_node()))
                .with_children(|builder| {
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
        });
}

fn title_container() -> NodeBundle {
    NodeBundle {
        style: Style {
            padding: UiRect::percent(30., 30., 5., 0.),
            ..Default::default()
        },
        ..Default::default()
    }
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

fn controls_panel_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            padding: UiRect::percent(30., 30., 0., 20.),
            ..Default::default()
        },
        ..Default::default()
    }
}

fn play_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
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

fn audio_slider() -> NodeBundle {
    NodeBundle {
        ..Default::default()
    }
}
