use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese_game::{
    AppState, Cheese, CheeseRacePlugin, Level, PlayerCameraPlugin, SceneAssets, SceneAssetsPlugin,
    TerrainNoise, TerrainPlugin,
};

fn main() {
    Test::new("Cheese controls".to_string(), |app| {
        app.add_plugins((
            PlayerCameraPlugin,
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

fn handle_start(mut commands: Commands, scenes: Res<SceneAssets>) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10.0e3,
            ..Default::default()
        },
        transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(Cheese::bundle(Cheese::default_transform(), &scenes));
    commands.spawn((Level::default(), Level::name()));
}
