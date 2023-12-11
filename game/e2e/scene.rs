use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::{components::ExternalImpulse, plugins::PhysicsDebugPlugin};

use cheese_game::{
    AppState, Cheese, CheeseAssetsPlugin, CheeseRacePlugin, Person, PlayerCameraPlugin,
    RaceScenePlugin, TerrainPlugin,
};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            PlayerCameraPlugin,
            CheeseRacePlugin,
            TerrainPlugin::default(),
            RaceScenePlugin,
            CheeseAssetsPlugin::new(AppState::SpawningScene),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(GlobalVolume::new(0.))
        .add_systems(Startup, spawn_ragdolls)
        .add_systems(Update, go_forward_input);
    })
    .run();
}

fn spawn_ragdolls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -3..=3 {
        Person::default().spawn_ragdoll(
            Vec3::new(x as f32, 10., -8.),
            Vec3::ZERO,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}

pub(crate) fn go_forward_input(
    inputs: Res<Input<KeyCode>>,
    mut query: Query<(&Transform, &mut ExternalImpulse), With<Cheese>>,
) {
    if inputs.pressed(KeyCode::Up) {
        for (transform, mut linear_impulse) in query.iter_mut() {
            let prev_impulse = linear_impulse.impulse();
            linear_impulse.set_impulse(prev_impulse + transform.forward() * -50.);
        }
    }
}
