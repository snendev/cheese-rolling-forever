// code adapted from
// https://github.com/Razaekel/noise-rs/blob/d79aa83cc5bab27ccab3c82cc9265add0bbeaa46/examples/complexplanet.rs

use noise::{Billow, Blend, Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti, ScaleBias};

const SEED: u32 = 54321;

pub fn generate_terrain_noise() -> impl NoiseFn<f64, 2> {
    let hilly_billow = ScaleBias::new(
        Billow::<Perlin>::new(SEED + 60)
            .set_frequency(0.008)
            .set_persistence(0.5)
            .set_lacunarity(2.162109375)
            .set_octaves(8),
    )
    .set_scale(3.0);
    let hilly_ridged_multi = ScaleBias::new(
        RidgedMulti::<Perlin>::new(SEED + 61)
            .set_frequency(2.0)
            .set_lacunarity(2.162109375)
            .set_octaves(1),
    )
    .set_scale(0.002);
    Blend::new(
        hilly_billow,
        hilly_ridged_multi,
        Fbm::<Perlin>::new(SEED + 2).set_frequency(0.001),
    )
}
