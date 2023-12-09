use bevy::prelude::*;

use cheese::{
    CheeseAssetsPlugin, CheeseRacePlugin, MenuPlugin, PlayerCameraPlugin, RaceScenePlugin,
    TerrainPlugin,
};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerCameraPlugin,
            CheeseRacePlugin,
            RaceScenePlugin,
            CheeseAssetsPlugin::default(),
            TerrainPlugin::new(rand::random()),
            MenuPlugin,
        ))
        .run();
}
