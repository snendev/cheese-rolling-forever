use bevy::prelude::*;

use crate::{AppState, ObstacleNoise};

mod systems;

#[derive(Debug)]
pub struct ObstaclesPlugin;

impl Plugin for ObstaclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            systems::update_obstacles.run_if(resource_exists::<ObstacleNoise>()),
        )
        .add_systems(OnEnter(AppState::SpawningScene), systems::seed_noise)
        .add_systems(Update, systems::attach_obstacles);
    }
}
