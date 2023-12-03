use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::{components::GravityScale, plugins::PhysicsDebugPlugin};

use cheese::{Cheese, CheeseGamePlugin, Person, RaceScenePlugin};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            CheeseGamePlugin,
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, spawn_ragdolls)
        .add_systems(Update, handle_start);
    })
    .run();
}

fn handle_start(inputs: Res<Input<KeyCode>>, mut q: Query<&mut GravityScale, With<Cheese>>) {
    if inputs.just_pressed(KeyCode::Space) {
        q.single_mut().0 = 1.;
    }
}

fn spawn_ragdolls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -10..=10 {
        Person::default().spawn_ragdoll(
            Vec3::X * (x as f32) + Vec3::Y * 2.,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}
