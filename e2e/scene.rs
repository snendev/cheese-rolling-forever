use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::{components::GravityScale, plugins::PhysicsDebugPlugin};

use cheese::{Cheese, CheeseGamePlugin, RaceScenePlugin};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            CheeseGamePlugin,
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Update, handle_start);
    })
    .run();
}

fn handle_start(inputs: Res<Input<KeyCode>>, mut q: Query<&mut GravityScale, With<Cheese>>) {
    if inputs.just_pressed(KeyCode::Space) {
        q.single_mut().0 = 1.;
    }
}
