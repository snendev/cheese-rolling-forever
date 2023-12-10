use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{despawn_all_recursive, AppState, Cheese, Terrain, TerrainChunk};

pub struct RaceScenePlugin;

impl Plugin for RaceScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::SpawningScene), spawn_scene)
            .add_systems(
                Update,
                begin_countdown.run_if(in_state(AppState::SpawningScene)),
            )
            .add_systems(
                OnEnter(AppState::Countdown),
                (ready_cheese, spawn_countdown_ui),
            )
            .add_systems(
                Update,
                (countdown_race, track_countdown_ui).run_if(in_state(AppState::Countdown)),
            )
            .add_systems(
                OnExit(AppState::Countdown),
                despawn_all_recursive::<CountdownUI>,
            )
            .add_systems(OnEnter(AppState::Racing), yeet_cheese);
    }
}

#[derive(Component)]
pub struct RaceCountdown(Timer);

const CHEESE_SPAWN_Z: f32 = 50.;

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10.0e3,
            ..Default::default()
        },
        transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
    commands.spawn(Terrain::default().to_bundle());
    commands.spawn((
        Name::new("Race Countdown Timer"),
        RaceCountdown(Timer::from_seconds(3., TimerMode::Once)),
    ));
    commands.spawn(Cheese::bundle(
        Transform::from_xyz(0., 50., CHEESE_SPAWN_Z),
        &mut meshes,
        &mut materials,
    ));
}

fn begin_countdown(
    query: Query<(&TerrainChunk, &Collider)>,
    mut state: ResMut<NextState<AppState>>,
) {
    // once some terrain exists with a collider, switch states
    if !query.is_empty() {
        state.set(AppState::Countdown);
    }
}

fn ready_cheese(
    spatial_query: SpatialQuery,
    mut cheese_query: Query<&mut Transform, With<Cheese>>,
    terrain_query: Query<&Transform, (With<TerrainChunk>, Without<Cheese>)>,
) {
    let Ok(mut cheese_transform) = cheese_query.get_single_mut() else {
        return;
    };

    if let Some(hit) = spatial_query.cast_ray(
        // TODO maybe derive this from the Terrain's chunk_size and quad_size
        Vec3::new(0., 10., CHEESE_SPAWN_Z),
        Vec3::NEG_Y,
        500.,
        false,
        SpatialQueryFilter::default(),
    ) {
        if !terrain_query.contains(hit.entity) {
            return;
        }
        let y = -hit.time_of_impact + 10.;
        let cheese_spawn_position = Vec3::new(0., y + Cheese::RADIUS * 2., CHEESE_SPAWN_Z);
        *cheese_transform = Transform::from_translation(cheese_spawn_position)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
    }
}

fn countdown_race(
    mut commands: Commands,
    mut countdown_query: Query<(Entity, &mut RaceCountdown)>,
    mut state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in countdown_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            commands.entity(entity).despawn();
            state.set(AppState::Racing);
        }
    }
}

fn yeet_cheese(mut cheese_query: Query<&mut ExternalImpulse, With<Cheese>>) {
    let mut impulse = cheese_query.single_mut();
    impulse.set_impulse(Vec3::Z * 4.);
}

#[derive(Component)]
struct CountdownUI;
#[derive(Component)]
struct CountdownUIText;

fn spawn_countdown_ui(mut commands: Commands) {
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

fn track_countdown_ui(
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
