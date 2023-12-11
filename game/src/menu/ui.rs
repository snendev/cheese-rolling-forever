use bevy::prelude::*;

use crate::{button, AppState};

const GAME_TITLE: &str = "Cheese";

#[derive(Component)]
pub(super) struct MenuUI;
#[derive(Component)]
pub(super) struct PlayButton;

pub(super) fn handle_play(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
    mut state: ResMut<NextState<AppState>>,
) {
    for interaction in interaction_query.iter() {
        if let Interaction::Pressed = interaction {
            state.set(AppState::SpawningScene);
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
                        .spawn((Name::new("Play Button"), PlayButton, button()))
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
