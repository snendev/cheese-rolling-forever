use noise::NoiseFn;

use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_resource::{Extent3d, PrimitiveTopology, TextureDimension, TextureFormat},
    },
};
use bevy_xpbd_3d::prelude::*;

mod noise_utils;
pub use noise_utils::generate_terrain_noise;

#[derive(Debug, Clone, Default)]
#[derive(Component)]
pub struct Terrain {
    pub mesh_builder: TerrainMeshBuilder,
    // the index of the maximum row currently rendered
    pub extents: (usize, usize),
    pub noise_seed: u32,
}

impl Terrain {
    pub fn new(start_length: usize) -> Self {
        Self {
            mesh_builder: TerrainMeshBuilder::default(),
            extents: (0, start_length),
            noise_seed: 0,
        }
    }

    pub fn with_seed(mut self, seed: u32) -> Self {
        self.noise_seed = seed;
        self
    }

    pub fn to_bundle(
        self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> impl Bundle {
        let noise = self.generate_noise();
        Self::to_bundle_with_noise(self, &noise, meshes, materials, images)
    }

    // N.B. this does not help when regenerating terrain with non-default noise
    pub fn to_bundle_with_noise(
        self,
        noise: &impl NoiseFn<f64, 2>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> impl Bundle {
        let mesh = self.generate_mesh(noise);
        (
            self,
            Name::new("Terrain"),
            RigidBody::Static,
            AsyncCollider(ComputedCollider::TriMesh),
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(images.add(uv_debug_texture())),
                    ..default()
                }),
                transform: Transform::default()
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_4)),
                ..Default::default()
            },
        )
    }

    pub fn generate_noise(&self) -> impl NoiseFn<f64, 2> {
        generate_terrain_noise(self.noise_seed)
    }

    pub fn extend(&mut self, rows_to_extend: usize) {
        self.extents.0 += rows_to_extend;
        self.extents.1 += rows_to_extend;
    }

    pub fn generate_mesh(&self, noise: &impl NoiseFn<f64, 2>) -> Mesh {
        let chunk = self.mesh_builder.generate_chunk(
            noise,
            self.extents.1 - self.extents.0,
            self.extents.0,
        );

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(chunk.indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, chunk.positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, chunk.normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, chunk.uvs)
    }
}

/// A rectangle on the `XY` plane centered at the origin.
/// Adapted from shape::Quad and shape::Plane to have a subdivided Quad
#[derive(Debug, Clone)]
pub struct TerrainMeshBuilder {
    // the quad size of each rendered chunk of the mesh
    pub quad_size: Vec2,
    // the horizontal grid width of the terrain
    pub vertices_per_row: u32,
}

impl Default for TerrainMeshBuilder {
    fn default() -> Self {
        TerrainMeshBuilder::new(Vec2::ONE * 2., 16)
    }
}

impl TerrainMeshBuilder {
    pub fn new(quad_size: Vec2, vertices_per_row: u32) -> Self {
        Self {
            quad_size,
            vertices_per_row,
        }
    }

    pub fn generate_chunk(
        &self,
        noise: &impl NoiseFn<f64, 2>,
        chunk_size: usize,
        start_index: usize,
    ) -> TerrainMeshChunk {
        let mut positions: Vec<[f32; 3]> =
            Vec::with_capacity(chunk_size * self.vertices_per_row as usize);
        let mut normals: Vec<[f32; 3]> =
            Vec::with_capacity(chunk_size * self.vertices_per_row as usize);
        let mut uvs: Vec<[f32; 2]> =
            Vec::with_capacity(chunk_size * self.vertices_per_row as usize);
        // Each row is (M - 1) X (N-1) quads
        let mut indices: Vec<u32> =
            Vec::with_capacity((chunk_size - 1) * (self.vertices_per_row as usize - 1) * 6);

        for z in start_index..(start_index + chunk_size) {
            for x in 0..self.vertices_per_row {
                let tx = (x as f32) / (self.vertices_per_row - 1) as f32 - 0.5;
                positions.push([
                    tx * (self.vertices_per_row as f32) * self.quad_size.x,
                    noise.get([x as f64, z as f64]) as f32,
                    (z as f32) * self.quad_size.y - self.quad_size.y * 0.5,
                ]);
                normals.push(Vec3::Y.to_array());
                // TODO: offsets for less repetitive uv?
                uvs.push([
                    tx,
                    (z as u32 % self.vertices_per_row) as f32 / self.vertices_per_row as f32,
                ]);
            }

            let adjusted_z = z as u32 - start_index as u32;
            if adjusted_z < chunk_size as u32 - 1 {
                for x in 0..(self.vertices_per_row - 1) {
                    let quad_index = self.vertices_per_row * adjusted_z + x;
                    // right triangle
                    indices.push(quad_index + self.vertices_per_row + 1);
                    indices.push(quad_index + 1);
                    indices.push(quad_index + self.vertices_per_row);
                    // left triangle
                    indices.push(quad_index);
                    indices.push(quad_index + self.vertices_per_row);
                    indices.push(quad_index + 1);
                }
            }
        }

        TerrainMeshChunk {
            positions,
            normals,
            uvs,
            indices,
        }
    }
}

// A chunk extends the terrain exactly one quad further, aka one "row" of vertices
#[derive(Debug)]
pub struct TerrainMeshChunk {
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub indices: Vec<u32>,
}

/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
        198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
