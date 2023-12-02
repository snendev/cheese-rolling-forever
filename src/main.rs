use bevy::prelude::*;

use cheese::{CheeseGamePlugin, RaceScenePlugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CheeseGamePlugin, RaceScenePlugin))
        .run();
}
