use bevy::{pbr::wireframe::WireframePlugin, prelude::*};

use bevy_geppetto::Test;

use bevy_xpbd_3d::{components::LinearVelocity, plugins::PhysicsDebugPlugin, resources::Gravity};

use cheese::{AppState, Cheese, CheeseRacePlugin, RaceScenePlugin, TerrainPlugin};

fn main() {
    Test::new("Terrain alone".to_string(), |app| {
        app.add_plugins((
            CheeseRacePlugin,
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
            WireframePlugin::default(),
            TerrainPlugin::default(),
        ))
        .insert_resource(Gravity(Vec3::ZERO))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, move_cheese)
        .add_systems(Startup, |mut state: ResMut<NextState<AppState>>| {
            state.set(AppState::SpawningScene);
        });
    })
    .run();
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(80., 0., -3.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn move_cheese(inputs: Res<Input<KeyCode>>, mut query: Query<&mut LinearVelocity, With<Cheese>>) {
    if let Ok(mut velocity) = query.get_single_mut() {
        if inputs.just_pressed(KeyCode::Space) {
            velocity.0 = Vec3::Z * 100.;
        }
    }
}
