use bevy_geppetto::Test;

use bevy_xpbd_3d::plugins::PhysicsDebugPlugin;

use cheese_game::{
    CheeseRacePlugin, MenuPlugin, PlayerCameraPlugin, RaceScenePlugin, SceneAssetsPlugin,
    TerrainPlugin,
};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            // but we can confirm that none of these are active, too
            PlayerCameraPlugin,
            CheeseRacePlugin,
            TerrainPlugin::default(),
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
            SceneAssetsPlugin::default(),
            // the thing we actually want to test
            MenuPlugin,
        ));
    })
    .run();
}
