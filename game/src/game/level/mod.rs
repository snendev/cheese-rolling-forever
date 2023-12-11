use itertools::Itertools;

use bevy::{prelude::*, utils::HashSet};

mod chunk;
pub use chunk::*;

mod plugin;
pub use plugin::*;

mod vertex;
pub use vertex::*;

// The grid extends the XZ plane into chunks and tracks which entities are inside which chunk
#[derive(Clone, Debug)]
#[derive(Component)]
pub struct Level {
    // in Chunk units
    pub chunks_in_play: HashSet<Vertex>,
    // in Grid units
    pub chunk_size: Vertex,
    pub quad_size: Vec2,
}

impl Level {
    const VISIBLE_CHUNKS_RANGE: (i32, i32) = (3, 3);

    pub fn new(chunk_size: Vertex, quad_size: Vec2) -> Self {
        Self {
            chunks_in_play: HashSet::default(),
            chunk_size,
            quad_size,
        }
    }

    pub fn name() -> Name {
        Name::new("Level")
    }

    pub fn update(&mut self, cheese_position: Vec3) {
        let cheese_chunk =
            Chunk::from_translation(cheese_position, self.chunk_size, self.quad_size);

        let left_edge = cheese_chunk
            .origin
            .x
            .saturating_sub(Self::VISIBLE_CHUNKS_RANGE.0);
        let right_edge = cheese_chunk
            .origin
            .x
            .saturating_add(Self::VISIBLE_CHUNKS_RANGE.0);
        let forward_edge = cheese_chunk
            .origin
            .z
            .saturating_add(Self::VISIBLE_CHUNKS_RANGE.1);
        let backward_edge = cheese_chunk
            .origin
            .z
            .saturating_sub(Self::VISIBLE_CHUNKS_RANGE.1);

        self.chunks_in_play.clear();
        for (x, y) in (left_edge..=right_edge).cartesian_product(backward_edge..=forward_edge) {
            let chunk_vertex = (x, y).into();
            self.chunks_in_play.insert(chunk_vertex);
        }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new(Vertex::new(40, 40), Vec2::ONE * 2.)
    }
}
