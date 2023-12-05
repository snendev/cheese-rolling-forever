use bevy::prelude::*;

use cheese::{CheeseGamePlugin, Person, PlayerCameraPlugin, RaceScenePlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerCameraPlugin,
            CheeseGamePlugin,
            RaceScenePlugin,
        ))
        .add_systems(Startup, spawn_ragdolls)
        .run();
}

fn spawn_ragdolls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -4..=4 {
        for y in -4..=4 {
            Person::default().spawn_ragdoll(
                Vec3::new(x as f32, 5. + (3. * y as f32), -8. + (-3. * y as f32)),
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
}
