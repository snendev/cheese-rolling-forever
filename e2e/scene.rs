use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese::{CheeseGamePlugin, Person, PlayerCameraPlugin, RaceScenePlugin};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            PlayerCameraPlugin,
            CheeseGamePlugin,
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, spawn_ragdolls);
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
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}
