use noise::NoiseFn;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use bevy_xpbd_3d::prelude::*;

use crate::TextureAssets;

#[derive(Debug, Clone)]
#[derive(Component)]
pub struct TerrainChunk {
    // the number of vertices in each chunk
    pub chunk_size: (u16, u16),
    // the origin in "vertex" coordinates of the chunk
    pub origin_vertex: (i32, i32),
    // the quad size of each rendered chunk of the mesh
    pub quad_size: Vec2,
    // TODO: a scaling factor on quad_size which draws fewer triangles for farther away chunks
    // pub lod_scale_factor: f32,
    pub noise_seed: u32,
}

impl Default for TerrainChunk {
    fn default() -> Self {
        TerrainChunk::new((0, 0), (40, 40), Vec2::ONE * 2., 0)
    }
}

impl TerrainChunk {
    pub fn new(
        origin_vertex: (i32, i32),
        chunk_size: (u16, u16),
        quad_size: Vec2,
        noise_seed: u32,
    ) -> Self {
        Self {
            quad_size,
            chunk_size,
            noise_seed,
            origin_vertex,
        }
    }

    pub fn generate_mesh(&self, noise: &impl NoiseFn<f64, 2>) -> Mesh {
        let num_vertices = self.chunk_size.0 * self.chunk_size.1;
        let num_indices = (self.chunk_size.0 - 1) * (self.chunk_size.1 - 1) * 6;
        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices as usize);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices as usize);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices as usize);
        // Each row is (M - 1) X (N-1) quads
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices as usize);

        let slope = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);

        // let total_z = self.origin_vertex.1 * self.chunk_size.1 as i32 + z;
        for z in 0..=self.chunk_size.1 as i32 {
            for x in 0..=self.chunk_size.0 as i32 {
                let tx = x as f32 / self.chunk_size.0 as f32;
                let x_position = (tx - 0.5) * self.chunk_size.0 as f32 * self.quad_size.x;
                let z_position = z as f32 * self.quad_size.y;

                let sample_x = (x + self.origin_vertex.0 * self.chunk_size.0 as i32) as f64;
                let sample_z = (self.chunk_size.1 as i32 - z
                    + self.origin_vertex.1 * self.chunk_size.1 as i32)
                    as f64;
                let noise_sample = noise.get([sample_x, sample_z]) as f32;
                let sloped_noise = slope * Vec3::new(0., noise_sample, 0.);

                let sloped_position = Vec3::new(x_position, -z_position, z_position);
                let unsloped_position = Vec3::new(x_position, 0., z_position);
                let target_position = sloped_position + sloped_noise;

                if self.origin_vertex.1 > 0 {
                    positions.push(unsloped_position.to_array());
                    normals.push(Vec3::Y.to_array());
                } else if self.origin_vertex.1 == 0 {
                    // blend between 0 and the noise
                    let chunk_z_ratio =
                        (self.chunk_size.1 as f32 - z as f32) / self.chunk_size.1 as f32;

                    positions.push(
                        target_position
                            .lerp(unsloped_position, chunk_z_ratio)
                            .to_array(),
                    );
                    normals.push(Vec3::Y.to_array());
                } else {
                    positions.push(target_position.to_array());
                    normals.push(Vec3::Y.to_array());
                }

                // TODO: offsets for less repetitive uv?
                uvs.push([
                    tx,
                    (z % (self.chunk_size.1 + 1) as i32) as f32 / self.chunk_size.1 as f32,
                ]);
            }

            if z < self.chunk_size.1 as i32 {
                for x in 0..self.chunk_size.0 {
                    let row_offset = self.chunk_size.0 as u32 + 1;
                    let quad_index = row_offset * z as u32 + x as u32;
                    // right triangle
                    indices.push(quad_index + row_offset + 1);
                    indices.push(quad_index + 1);
                    indices.push(quad_index + row_offset);
                    // left triangle
                    indices.push(quad_index);
                    indices.push(quad_index + row_offset);
                    indices.push(quad_index + 1);
                }
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
        let x = self.origin_vertex.0 as f32 * self.chunk_size.0 as f32 * self.quad_size.x;
        let y = (self.origin_vertex.1 as f32).clamp(std::f32::NEG_INFINITY, 0.)
            * self.chunk_size.1 as f32
            * self.quad_size.y;
        let z = -(self.origin_vertex.1 as f32 * self.chunk_size.1 as f32) * self.quad_size.y;
        (
            Name::new(format!(
                "Terrain Chunk {}x{}",
                self.origin_vertex.0, self.origin_vertex.1,
            )),
            RigidBody::Static,
            ColliderDensity(1e7),
            AsyncCollider(ComputedCollider::TriMesh),
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(textures.ground.clone()),
                    depth_map: Some(textures.ground_displacement.clone()),
                    normal_map_texture: Some(textures.ground_normal.clone()),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(x, y, z),
                ..Default::default()
            },
            self,
        )
    }
}
