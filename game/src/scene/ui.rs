use bevy::prelude::*;

use crate::{button, AppState, HighScore, Score};

use super::RaceCountdown;

#[derive(Component)]
pub(super) struct CountdownUI;
#[derive(Component)]
pub(super) struct CountdownUIText;

pub(super) fn spawn_countdown_ui(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Countdown UI"),
            CountdownUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                CountdownUIText,
                TextBundle::from_section(
                    "3",
                    TextStyle {
                        font_size: 512.0,
                        color: Color::rgb(0.02, 0.02, 0.1),
                        ..Default::default()
                    },
                ),
            ));
        });
}

pub(super) fn track_countdown_ui(
    mut ui_query: Query<&mut Text, With<CountdownUIText>>,
    countdown_query: Query<&RaceCountdown>,
) {
    let Ok(mut ui_text) = ui_query.get_single_mut() else {
        return;
    };
    let Ok(countdown) = countdown_query.get_single() else {
        return;
    };
    ui_text.sections[0].value = format!(
        "{}",
        (countdown.0.duration() - countdown.0.elapsed()).as_secs() + 1
    );
}

#[derive(Component)]
pub(super) struct GameOverUI;

pub(super) fn spawn_game_over_ui(
    mut commands: Commands,
    score: Res<Score>,
    high_score: Res<HighScore>,
) {
    commands
        .spawn((
            Name::new("Game Over UI"),
            GameOverUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    // background_color,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Px(400.),
                            height: Val::Px(400.),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::SpaceEvenly,
                            align_items: AlignItems::Center,
                            row_gap: Val::Px(8.),
                            border: UiRect::all(Val::Px(6.)),
                            ..Default::default()
                        },
                        background_color: Color::rgb(0.8, 0.8, 0.95).into(),
                        border_color: Color::rgb(0., 0., 0.).into(),
                        ..Default::default()
                    },
                    Name::new("Game Over Panel"),
                ))
                .with_children(|builder| {
                    builder.spawn(TextBundle::from_section(
                        "Game Over!",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::rgb(0.02, 0.02, 0.1),

                            ..Default::default()
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        "Your score is:",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::rgb(0.02, 0.02, 0.1),
                            ..Default::default()
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        format!("{:.0}", score.0),
                        TextStyle {
                            font_size: 24.0,
                            color: Color::rgb(0.02, 0.02, 0.1),
                            ..Default::default()
                        },
                    ));
                    if score.0 > high_score.0 {
                        builder.spawn(TextBundle::from_section(
                            "New high score!",
                            TextStyle {
                                font_size: 28.0,
                                color: Color::rgb(0.02, 0.02, 0.1),
                                ..Default::default()
                            },
                        ));
                    }
                    builder.spawn(TextBundle::from_section(
                        "Previous high score:",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::rgb(0.02, 0.02, 0.1),
                            ..Default::default()
                        },
                    ));
                    builder.spawn(TextBundle::from_section(
                        format!("{:.0}", high_score.0),
                        TextStyle {
                            font_size: 24.0,
                            color: Color::rgb(0.02, 0.02, 0.1),
                            ..Default::default()
                        },
                    ));
                    builder
                        .spawn((Name::new("Replay Button"), ReplayButton, button()))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("Replay Button Text"),
                                TextBundle::from_section(
                                    "Play Again",
                                    TextStyle {
                                        font_size: 28.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..Default::default()
                                    },
                                ),
                            ));
                        });
                    builder
                        .spawn((Name::new("Quit Button"), QuitButton, button()))
                        .with_children(|parent| {
                            parent.spawn((
                                Name::new("Play Again Button Text"),
                                TextBundle::from_section(
                                    "Quit",
                                    TextStyle {
                                        font_size: 32.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                        ..Default::default()
                                    },
                                ),
                            ));
                        });
                });
        });
}

#[derive(Component)]
pub(super) struct ReplayButton;
#[derive(Component)]
pub(super) struct QuitButton;

pub(super) fn handle_replay_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ReplayButton>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Pressed = interaction {
            state.set(AppState::SpawningScene);
        }
    }
}

pub(super) fn handle_quit_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Pressed = interaction {
            state.set(AppState::Menu);
        }
    }
}
