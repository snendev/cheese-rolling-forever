use noise::{MultiFractal, NoiseFn, Perlin, RidgedMulti};

use bevy::prelude::*;

#[derive(Resource)]
pub struct ObstacleNoise(Box<dyn NoiseFn<f64, 2> + Send + Sync>);

impl ObstacleNoise {
    pub fn new(seed: u32) -> Self {
        Self::from_noise(Self::generate_noise(seed))
    }

    pub fn from_noise(noise: impl NoiseFn<f64, 2> + Send + Sync + 'static) -> Self {
        Self(Box::new(noise))
    }

    pub fn get(&self) -> &dyn NoiseFn<f64, 2> {
        &self.0
    }

    fn generate_noise(seed: u32) -> impl NoiseFn<f64, 2> {
        RidgedMulti::<Perlin>::new(seed + 17)
            .set_frequency(4.0)
            .set_lacunarity(2.162109375)
            .set_octaves(2)
    }
}

impl Default for ObstacleNoise {
    fn default() -> Self {
        Self::new(54321)
    }
}
