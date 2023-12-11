use ::noise::NoiseFn;
use rand::Rng;

use bevy::{prelude::*, utils::HashMap};

use crate::{Level, TextureAssets, Vertex};

use super::Chunk;

mod noise;
pub use noise::*;

mod plugin;
pub use plugin::*;

mod wall;
pub use wall::*;

pub struct Obstacle;

#[derive(Debug, Clone, Default)]
#[derive(Component)]
pub struct Obstacles {
    pub chunk_entities: HashMap<Vertex, Vec<Entity>>,
}

impl Obstacles {
    pub fn new() -> Self {
        Self {
            chunk_entities: HashMap::default(),
        }
    }

    pub fn generate_obstacles_for_chunk<'a>(
        &'a self,
        chunk: Chunk,
        noise: &'a impl NoiseFn<f64, 2>,
        textures: &'a TextureAssets,
        meshes: &'a mut Assets<Mesh>,
        materials: &'a mut Assets<StandardMaterial>,
    ) -> impl Iterator<Item = impl Bundle> + 'a {
        let mut rng = rand::thread_rng();
        let before_first_chunk = chunk.origin.z <= 0;
        let before_fifth_chunk = chunk.origin.z < 5;
        chunk
            .iter_by_row()
            // don't iterate along the final edge
            .take(((chunk.size.z - 1) * chunk.size.x) as usize)
            // skip by some arbitrary amount
            .step_by(rng.gen_range((chunk.size.x * 4)..(chunk.size.x * 4 + 7)) as usize)
            // and make a wall there if the noise value is high enough
            .filter_map(move |vertex| {
                if before_first_chunk {
                    return None;
                }
                let global_vertex = chunk.to_global_coords(vertex);
                let position = chunk.to_translation(global_vertex);
                let noise = noise.get([position.x as f64, position.y as f64]);
                let noise_threshold = if before_fifth_chunk { 0.995 } else { 0.98 };
                if noise > noise_threshold {
                    info!("{:?} {} {}", global_vertex, position, noise);
                    Some(
                        Wall::new(
                            chunk.clone(),
                            vertex,
                            Vec2::new(
                                chunk.quad_size.x * chunk.size.x as f32 / 2.,
                                chunk.quad_size.y,
                            ),
                        )
                        .to_bundle(textures, meshes, materials),
                    )
                } else {
                    None
                }
            })
    }

    pub fn update(
        &mut self,
        level: &Level,
        noise: &impl NoiseFn<f64, 2>,
        commands: &mut Commands,
        textures: &TextureAssets,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        // remove out-of-bounds chunks
        let chunks_to_remove = self
            .chunk_entities
            .iter()
            .filter_map(|(vertex, _)| {
                if !level.chunks_in_play.contains(vertex) {
                    Some(*vertex)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for vertex in chunks_to_remove {
            if let Some(entities) = self.chunk_entities.remove(&vertex) {
                for entity in entities {
                    commands.entity(entity).despawn();
                }
            }
        }

        // spawn missing in-bounds chunks
        for origin in level.chunks_in_play.iter() {
            if !self.chunk_entities.contains_key(origin) {
                let chunk = Chunk {
                    quad_size: level.quad_size,
                    size: level.chunk_size,
                    // IDK why this works but it does
                    origin: Vertex::new(origin.x, -origin.z),
                };
                let mut chunk_entities = vec![];
                for bundle in
                    self.generate_obstacles_for_chunk(chunk, noise, textures, meshes, materials)
                {
                    let entity = commands.spawn(bundle).id();
                    chunk_entities.push(entity)
                }
                self.chunk_entities.insert(*origin, chunk_entities);
            }
        }
    }
}
