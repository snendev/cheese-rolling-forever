use bevy::prelude::*;

use cheese::{CheeseRacePlugin, MenuPlugin, PlayerCameraPlugin, RaceScenePlugin, TerrainPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerCameraPlugin,
            CheeseRacePlugin,
            RaceScenePlugin,
            TerrainPlugin::new(rand::random()),
            MenuPlugin,
        ))
        .run();
}
