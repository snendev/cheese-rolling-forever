use ::noise::NoiseFn;

use bevy::{prelude::*, utils::HashMap};

mod chunk;
pub use chunk::*;

mod noise;
pub use noise::*;

mod plugin;
pub use plugin::*;

use crate::{Chunk, Level, TextureAssets, Vertex};

#[derive(Clone, Debug, Default)]
#[derive(Component)]
pub struct Terrain {
    pub chunk_entities: HashMap<Vertex, Vec<Entity>>,
}

impl Terrain {
    pub fn new() -> Self {
        Self {
            chunk_entities: HashMap::new(),
        }
    }

    pub fn name() -> Name {
        Name::new("Terrain")
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
                    origin: *origin,
                };
                let chunk_bundle =
                    TerrainChunk::new(chunk).to_bundle(noise, textures, meshes, materials);
                let chunk_entity = commands.spawn(chunk_bundle).id();
                self.chunk_entities.insert(*origin, vec![chunk_entity]);
            }
        }
    }
}
