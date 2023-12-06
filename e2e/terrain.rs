use bevy::{pbr::wireframe::WireframePlugin, prelude::*};

use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese::{CheeseGamePlugin, RaceScenePlugin};

fn main() {
    Test::new("Terrain alone".to_string(), |app| {
        app.add_plugins((
            CheeseGamePlugin,
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
            WireframePlugin::default(),
        ))
        .add_systems(Startup, spawn_scene);
    })
    .run();
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 50., -3.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
