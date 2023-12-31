use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese_game::{
    AppState, Cheese, CheeseRacePlugin, Level, Person, SceneAssetsPlugin, TerrainNoise,
    TerrainPlugin,
};

fn main() {
    Test::new("Cheese controls".to_string(), |app| {
        app.add_plugins((
            CheeseRacePlugin,
            PhysicsDebugPlugin::default(),
            TerrainPlugin,
            SceneAssetsPlugin::new(AppState::SpawningScene),
        ))
        .insert_resource(TerrainNoise::from_noise(noise::Constant::new(0.)))
        .add_systems(Startup, handle_start);
    })
    .run();
}

fn handle_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2., 10., -6.).looking_at(Vec3::new(0., 5., -8.), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10.0e3,
            ..Default::default()
        },
        transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn((Level::default(), Level::name()));

    for (x, y) in (0..1).zip(0..1) {
        Person::default().spawn_ragdoll(
            Vec3::new(4. * x as f32, 5. + (4. * y as f32), -8. + (4. * y as f32)),
            Vec3::ZERO,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }

    commands.spawn((Cheese, SpatialBundle::default()));
}
