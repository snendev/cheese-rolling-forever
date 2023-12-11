use bevy::prelude::*;

use crate::{AppState, TerrainNoise, TextureAssets};

mod systems;

#[derive(Debug)]
pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            systems::update_terrain_mesh.run_if(
                resource_exists::<TextureAssets>().and_then(resource_exists::<TerrainNoise>()),
            ),
        )
        .add_systems(OnEnter(AppState::SpawningScene), systems::seed_noise)
        .add_systems(Update, systems::attach_terrain);
    }
}
