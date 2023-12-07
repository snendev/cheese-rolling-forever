use bevy::prelude::*;

use crate::TerrainNoise;

mod systems;

#[derive(Debug, Default)]
pub struct TerrainPlugin {
    noise_seed: u32,
}

impl TerrainPlugin {
    pub fn new(noise_seed: u32) -> Self {
        Self { noise_seed }
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainNoise::new(self.noise_seed))
            .add_systems(Update, systems::update_terrain_mesh);
    }
}
