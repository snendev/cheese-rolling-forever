use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese::{CheeseRacePlugin, MenuPlugin, PlayerCameraPlugin, RaceScenePlugin, TerrainPlugin};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            // but we can confirm that none of these are active, too
            PlayerCameraPlugin,
            CheeseRacePlugin,
            TerrainPlugin::default(),
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
            // the thing we actually want to test
            MenuPlugin,
        ));
    })
    .run();
}
