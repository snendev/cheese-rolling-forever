use noise::NoiseFn;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_xpbd_3d::prelude::*;

use crate::{Chunk, TextureAssets, Vertex};

#[derive(Debug, Clone)]
#[derive(Component)]
pub struct TerrainChunk {
    // the chunk being rendered
    pub chunk: Chunk,
    // TODO: a scaling factor on chunk.quad_size which draws fewer triangles for farther away chunks
    // pub lod_scale_factor: f32,
    pub noise_seed: u32,
}

impl Default for TerrainChunk {
    fn default() -> Self {
        TerrainChunk::new(
            Chunk::new((0, 0).into(), (40, 40).into(), Vec2::ONE * 2.),
            0,
        )
    }
}

impl TerrainChunk {
    pub fn new(chunk: Chunk, noise_seed: u32) -> Self {
        Self { chunk, noise_seed }
    }

    // get the triangles to render the quad with origin at local_vertex
    pub fn get_quad_triangles(&self, local_vertex: Vertex) -> [u32; 6] {
        let row_offset = self.chunk.size.x as u32 + 1;
        let quad_index = row_offset * local_vertex.z as u32 + local_vertex.x as u32;
        [
            // right triangle
            quad_index + row_offset + 1,
            quad_index + 1,
            quad_index + row_offset,
            // left triangle
            quad_index,
            quad_index + row_offset,
            quad_index + 1,
        ]
    }

    pub fn generate_mesh(&self, noise: &impl NoiseFn<f64, 2>) -> Mesh {
        let num_vertices = self.chunk.count_vertices() as usize;
        let num_indices = self.chunk.count_indices() as usize;
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        // Each row is (M - 1) X (N-1) quads
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices);

        let slope = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);

        for vertex in self.chunk.iter_by_row() {
            let global_vertex = self.chunk.to_global_coords(vertex);
            let noise_sample = noise.get([global_vertex.x as f64, global_vertex.z as f64]) as f32;
            let sloped_noise = slope * Vec3::new(0., noise_sample, 0.);
            let position = self.chunk.to_translation(vertex);
            let sloped_position = Vec3::new(position.x, -position.y, position.y);
            let unsloped_position = Vec3::new(position.x, 0., position.y);
            let target_position = sloped_position + sloped_noise;

            match self.chunk.origin.z.cmp(&0) {
                std::cmp::Ordering::Less => {
                    positions.push(target_position.to_array());
                    normals.push(Vec3::Y.to_array());
                }
                std::cmp::Ordering::Equal => {
                    // blend between 0 and the noise
                    let chunk_z_ratio =
                        (self.chunk.size.z as f32 - vertex.z as f32) / self.chunk.size.z as f32;

                    positions.push(
                        target_position
                            .lerp(unsloped_position, chunk_z_ratio)
                            .to_array(),
                    );
                    normals.push(Vec3::Y.to_array());
                }
                std::cmp::Ordering::Greater => {
                    positions.push(unsloped_position.to_array());
                    normals.push(Vec3::Y.to_array());
                }
            }

            uvs.push([global_vertex.z as f32 / 8., global_vertex.x as f32 / 8.]);

            if vertex.x < self.chunk.size.x && vertex.z < self.chunk.size.z {
                indices.extend_from_slice(&self.get_quad_triangles(vertex));
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }

    pub fn to_bundle(
        self,
        noise: &impl NoiseFn<f64, 2>,
        textures: &TextureAssets,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> impl Bundle {
        let mesh = self.generate_mesh(noise);
        let x = self.chunk.origin.x as f32 * self.chunk.size.x as f32 * self.chunk.quad_size.x;
        let y = (self.chunk.origin.z as f32).clamp(std::f32::NEG_INFINITY, 0.)
            * self.chunk.size.z as f32
            * self.chunk.quad_size.y;
        let z = -(self.chunk.origin.z as f32 * self.chunk.size.z as f32) * self.chunk.quad_size.y;
        (
            Name::new(format!(
                "Terrain Chunk {}x{}",
                self.chunk.origin.x, self.chunk.origin.z,
            )),
            RigidBody::Static,
            ColliderDensity(1e7),
            AsyncCollider(ComputedCollider::TriMesh),
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(textures.ground.clone()),
                    normal_map_texture: Some(textures.ground_normal.clone()),
                    thickness_texture: Some(textures.ground_displacement.clone()),
                    depth_map: Some(textures.ground_displacement.clone()),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            },
            self,
        )
    }
}
