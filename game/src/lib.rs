use bevy::{asset::AssetMetaCheck, prelude::*};

mod assets;
pub use assets::*;

mod game;
pub use game::*;

mod menu;
pub use menu::*;

mod scene;
pub use scene::*;

mod systems;
pub use systems::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    SpawningScene,
    Countdown,
    Racing,
    Closing,
}

pub fn run_app(canvas: Option<String>) {
    App::default()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas,
                resolution: (1366., 768.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            PlayerCameraPlugin,
            CheeseRacePlugin,
            RaceScenePlugin,
            CheeseAssetsPlugin::default(),
            TerrainPlugin::new(rand::random()),
            MenuPlugin,
        ))
        .run();
}
