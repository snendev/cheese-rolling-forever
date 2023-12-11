use itertools::Itertools;

use bevy::prelude::*;

use super::Vertex;

// Handles chunking in 2D vertices using a consistent grid of `Vertex` in quad_size units
#[derive(Debug, Clone)]
#[derive(Reflect)]
pub struct Chunk {
    // the length of the chunk in vertices
    pub size: Vertex,
    // the chunk's origin in chunk coordinates
    pub origin: Vertex,
    // the quad size of each chunk in the transform space
    pub quad_size: Vec2,
}

impl Chunk {
    pub fn new(origin: Vertex, size: Vertex, quad_size: Vec2) -> Self {
        Self {
            size,
            origin,
            quad_size,
        }
    }

    // gets the nearest chunk to a given translation
    pub fn from_translation(translation: Vec3, chunk_size: Vertex, quad_size: Vec2) -> Self {
        let origin = Vertex::from_translation(
            translation,
            // scale quads by chunk_size to get position in chunk units rather than grid units
            quad_size * Vec2::new(chunk_size.x as f32, chunk_size.z as f32),
        );
        Self::new(origin, chunk_size, quad_size)
    }

    pub fn count_vertices(&self) -> i32 {
        (self.size.x + 1) * (self.size.z + 1)
    }

    pub fn count_indices(&self) -> i32 {
        self.area() * 6
    }

    pub fn area(&self) -> i32 {
        self.size.x * self.size.z
    }

    // IDK pal
    pub fn to_other_global_coords(&self, local_vertex: Vertex) -> Vertex {
        let global_vx = local_vertex.x + self.origin.x * self.size.x;
        let global_vz = self.size.z + self.origin.z * self.size.z;
        Vertex {
            x: global_vx,
            z: global_vz,
        }
    }

    pub fn to_global_coords(&self, local_vertex: Vertex) -> Vertex {
        let global_vx = local_vertex.x + self.origin.x * self.size.x;
        let global_vz = self.size.z - local_vertex.z + self.origin.z * self.size.z;
        Vertex {
            x: global_vx,
            z: global_vz,
        }
    }

    pub fn to_translation(&self, vertex: Vertex) -> Vec2 {
        Vec2::new(
            vertex.x as f32 * self.quad_size.x,
            vertex.z as f32 * self.quad_size.y,
        )
    }

    pub fn iter_by_row(&self) -> impl Iterator<Item = Vertex> {
        (0..=self.size.z)
            .cartesian_product(0..=self.size.x)
            .into_iter()
            .map(|(z, x)| Vertex { x, z })
    }
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new((0, 0).into(), (40, 40).into(), Vec2::ONE * 2.)
    }
}
