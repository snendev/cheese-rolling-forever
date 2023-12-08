use bevy::prelude::*;

use cheese::{CheeseGamePlugin, Person, PlayerCameraPlugin, RaceScenePlugin, TerrainPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerCameraPlugin,
            CheeseGamePlugin,
            RaceScenePlugin,
            TerrainPlugin::default(),
        ))
        .add_systems(Startup, spawn_ragdolls)
        .run();
}

fn spawn_ragdolls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for x in -2..=2 {
        for y in -2..=2 {
            Person::default().spawn_ragdoll(
                Vec3::new(6. * x as f32, 5. + (4. * y as f32), 8. + (4. * y as f32)),
                &mut commands,
                &mut meshes,
                &mut materials,
            );
        }
    }
}
