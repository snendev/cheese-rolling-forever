use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::AppState;

mod camera;
pub use camera::*;

mod cheese;
pub use cheese::*;

mod person;
pub use person::*;

mod terrain;
pub use terrain::*;

mod systems;

pub struct CheeseRacePlugin;

impl Plugin for CheeseRacePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugins(PhysicsPlugins::default())
            .configure_sets(
                PostUpdate,
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .run_if(in_state(AppState::Racing)),
            )
            .add_systems(
                Update,
                (
                    systems::handle_inputs,
                    systems::chase_cheese,
                    systems::detect_grab,
                    systems::spawn_ragdolls,
                    systems::loop_ragdolls,
                    systems::despawn_infinites,
                )
                    .run_if(in_state(AppState::Racing)),
            );
    }
}

pub struct RaceScenePlugin;

impl Plugin for RaceScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Starting), spawn_scene)
            .add_systems(Update, start_race.run_if(in_state(AppState::Starting)))
            .add_systems(OnEnter(AppState::Racing), yeet_cheese);
    }
}

#[derive(Component)]
pub struct RaceCountdown(Timer);

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
    commands.spawn(Cheese::bundle(
        Transform::from_translation(Vec3::Y * Cheese::RADIUS + 0.01)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(Terrain::new((10, 40)).to_bundle());
    commands.spawn((
        Name::new("Race Countdown Timer"),
        RaceCountdown(Timer::from_seconds(3., TimerMode::Once)),
    ));
}

fn start_race(
    mut commands: Commands,
    mut countdown_query: Query<(Entity, &mut RaceCountdown)>,
    mut state: ResMut<NextState<AppState>>,
    time: Res<Time>,
) {
    for (entity, mut timer) in countdown_query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            info!("Ready, set, go!");
            commands.entity(entity).despawn();
            state.set(AppState::Racing);
        }
    }
}

fn yeet_cheese(mut cheese_query: Query<&mut ExternalImpulse, With<Cheese>>) {
    let mut impulse = cheese_query.single_mut();
    impulse.set_impulse(Vec3::Z * 4.);
}
