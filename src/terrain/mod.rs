use itertools::Itertools;

use bevy::{prelude::*, utils::HashMap};
use bevy_xpbd_3d::prelude::*;

mod chunk;
pub use chunk::*;

mod noise;
pub use noise::*;

mod plugin;
use ::noise::NoiseFn;
pub use plugin::*;

#[derive(Debug, Clone, Default)]
#[derive(Component)]
pub struct Terrain {
    pub chunk_size: (u16, u16),
    pub quad_size: Vec2,
    pub noise_seed: u32,
    pub rendered_chunks: HashMap<(i32, i32), Entity>,
}

impl Terrain {
    pub const DEFAULT_SEED: u32 = 54321;
    const VISIBLE_CHUNKS_RANGE: (i32, i32) = (2, 3);

    pub fn new(chunk_size: (u16, u16)) -> Self {
        Self {
            chunk_size,
            quad_size: Vec2::ONE * 2.,
            noise_seed: Self::DEFAULT_SEED,
            rendered_chunks: HashMap::new(),
        }
    }

    pub fn with_seed(mut self, noise_seed: u32) -> Self {
        self.noise_seed = noise_seed;
        self
    }

    pub fn to_bundle(self) -> impl Bundle {
        (self, Name::new("Terrain"), RigidBody::Static)
    }

    pub fn generate_chunk(
        &self,
        chunk_origin: (i32, i32),
        noise: &impl NoiseFn<f64, 2>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> impl Bundle {
        TerrainChunk {
            quad_size: self.quad_size,
            chunk_size: self.chunk_size,
            origin_vertex: chunk_origin,
            noise_seed: self.noise_seed,
        }
        .to_bundle(noise, meshes, materials, images)
    }

    pub fn update(
        &mut self,
        cheese_position: Vec3,
        noise: &impl NoiseFn<f64, 2>,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) {
        let cheese_nearest_vertex = Vec2::new(cheese_position.x, -cheese_position.z)
            / (self.quad_size * Vec2::new(self.chunk_size.0 as f32, self.chunk_size.1 as f32));
        let (cheese_x, cheese_y) = (
            cheese_nearest_vertex.x.round() as i32,
            cheese_nearest_vertex.y.round() as i32,
        );

        let left_edge = cheese_x.saturating_sub(Self::VISIBLE_CHUNKS_RANGE.0);
        let right_edge = cheese_x.saturating_add(Self::VISIBLE_CHUNKS_RANGE.0);
        let forward_edge = cheese_y.saturating_add(Self::VISIBLE_CHUNKS_RANGE.1);
        let backward_edge = cheese_y.saturating_sub(Self::VISIBLE_CHUNKS_RANGE.1);

        // remove out-of-bounds chunks

        let chunks_to_remove = self
            .rendered_chunks
            .iter()
            .filter_map(|((chunk_x, chunk_y), entity)| {
                if *chunk_x < left_edge
                    || *chunk_x > right_edge
                    || *chunk_y < backward_edge
                    || *chunk_y > forward_edge
                {
                    Some(((*chunk_x, *chunk_y), *entity))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for ((chunk_x, chunk_y), entity) in chunks_to_remove {
            self.rendered_chunks.remove(&(chunk_x, chunk_y));
            commands.entity(entity).despawn();
        }

        // spawn missing in-bounds chunks
        for (x, y) in (left_edge..=right_edge).cartesian_product(backward_edge..=forward_edge) {
            if !self.rendered_chunks.contains_key(&(x, y)) {
                let chunk_bundle = self.generate_chunk((x, y), noise, meshes, materials, images);
                let chunk_entity = commands.spawn(chunk_bundle).id();
                self.rendered_chunks.insert((x, y), chunk_entity);
            }
        }
    }
}
