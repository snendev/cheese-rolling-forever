use bevy::prelude::*;

use bevy_xpbd_3d::components::GravityScale;

use cheese::{Cheese, CheeseGamePlugin, Person, RaceScenePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CheeseGamePlugin, RaceScenePlugin))
        .add_systems(Startup, spawn_ragdolls)
        .add_systems(Update, handle_start)
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
            Vec3::new(x as f32, 10., -8.),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }
}
