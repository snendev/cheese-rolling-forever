use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese::{Cheese, CheeseGamePlugin, PlayerCameraPlugin, Terrain};

fn main() {
    Test::new("Cheese controls".to_string(), |app| {
        app.add_plugins((
            PlayerCameraPlugin,
            CheeseGamePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, handle_start);
    })
    .run();
}

fn handle_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10.0e3,
            ..Default::default()
        },
        transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(Cheese::bundle(&mut meshes, &mut materials));
    commands.spawn(Terrain::new(100).to_bundle_with_noise(
        &noise::Constant::new(0.),
        &mut meshes,
        &mut materials,
        &mut images,
    ));
}
