use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::{PhysicsDebugPlugin, PhysicsPlugins};

use cheese::RaceScenePlugin;

fn main() {
    Test::new("Terrain alone".to_string(), |app| {
        app.add_plugins((
            PhysicsPlugins::default(),
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Startup, spawn_scene);
    })
    .run();
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
