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

pub struct CheeseGamePlugin;

impl Plugin for CheeseGamePlugin {
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
            .add_systems(OnEnter(AppState::Starting), start_race);
    }
}

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
        Cheese::default_transform(),
        &mut meshes,
        &mut materials,
    ));
    commands.spawn(Terrain::new((10, 40)).to_bundle());
}

fn start_race(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::Racing);
}
