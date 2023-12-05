use noise::{Billow, Blend, Fbm, MultiFractal, NoiseFn, Perlin, RidgedMulti};

const SEED: u32 = 54321;

pub(super) fn generate_terrain_noise() -> impl NoiseFn<f64, 2> {
    Billow::<Perlin>::new(SEED + 60)
        // .set_frequency(1663.0)
        // .set_persistence(0.5)
        // .set_lacunarity(HILLS_LACUNARITY)
        // .set_octaves(6)
        ;
    Blend::new(
        Perlin::new(SEED),
        RidgedMulti::<Perlin>::new(SEED + 1),
        Fbm::<Perlin>::new(SEED + 2),
    )
}
