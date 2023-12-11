use bevy::prelude::*;

use crate::{despawn_all_recursive, AppState, Cheese};

#[derive(Clone, Copy, Debug, Default)]
#[derive(Resource)]
pub struct Score(pub f32);
#[derive(Clone, Copy, Debug, Default)]
#[derive(Resource)]
pub struct HighScore(pub f32);

#[derive(Clone, Copy, Debug)]
#[derive(Component)]
pub struct ScoreUI;
#[derive(Clone, Copy, Debug)]
#[derive(Component)]
pub struct ScoreText;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .init_resource::<HighScore>()
            .add_systems(OnEnter(AppState::SpawningScene), render_score_ui)
            .add_systems(
                Update,
                (track_score, track_score_ui).run_if(in_state(AppState::Racing)),
            )
            .add_systems(
                OnExit(AppState::Racing),
                (apply_deferred, despawn_all_recursive::<ScoreUI>).chain(),
            )
            .add_systems(OnExit(AppState::GameOver), update_scores);
    }
}

fn track_score(mut score: ResMut<Score>, cheese_query: Query<&Transform, With<Cheese>>) {
    let Ok(transform) = cheese_query.get_single() else {
        return;
    };

    score.0 = transform.translation.z - 50.;
}

fn update_scores(mut score: ResMut<Score>, mut high_score: ResMut<HighScore>) {
    if score.0 > high_score.0 {
        high_score.0 = score.0;
    }
    score.0 = 0.;
}

fn render_score_ui(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Score UI"),
            ScoreUI,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Percent(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn((
                Name::new("Score Text"),
                ScoreText,
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font_size: 64.,
                        ..Default::default()
                    },
                ),
            ));
        });
}

fn track_score_ui(mut ui_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    let Ok(mut text) = ui_query.get_single_mut() else {
        return;
    };
    text.sections[0].value = format!("{:.0}", score.0);
}
