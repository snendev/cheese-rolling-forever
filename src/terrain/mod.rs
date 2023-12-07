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
    pub extents: (u32, u32),
    pub noise_seed: u32,
}

impl Terrain {
    pub fn new(start_length: u32) -> Self {
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
                ..Default::default()
            },
        )
    }

    pub fn generate_noise(&self) -> impl NoiseFn<f64, 2> {
        generate_terrain_noise(self.noise_seed)
    }

    pub fn extend(&mut self, rows_to_extend: u32) {
        self.extents.0 += rows_to_extend;
        self.extents.1 += rows_to_extend;
    }

    pub fn generate_mesh(&self, noise: &impl NoiseFn<f64, 2>) -> Mesh {
        self.mesh_builder
            .generate_mesh(noise, self.extents.1 - self.extents.0, self.extents.0)
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
        TerrainMeshBuilder::new(Vec2::ONE * 2., 64)
    }
}

impl TerrainMeshBuilder {
    pub fn new(quad_size: Vec2, vertices_per_row: u32) -> Self {
        Self {
            quad_size,
            vertices_per_row,
        }
    }

    pub fn generate_mesh(
        &self,
        noise: &impl NoiseFn<f64, 2>,
        chunk_size: u32,
        start_index: u32,
    ) -> Mesh {
        let mut positions: Vec<[f32; 3]> =
            Vec::with_capacity((chunk_size * self.vertices_per_row) as usize);
        let mut normals: Vec<[f32; 3]> =
            Vec::with_capacity((chunk_size * self.vertices_per_row) as usize);
        let mut uvs: Vec<[f32; 2]> =
            Vec::with_capacity((chunk_size * self.vertices_per_row) as usize);
        // Each row is (M - 1) X (N-1) quads
        let mut indices: Vec<u32> =
            Vec::with_capacity(((chunk_size - 1) * (self.vertices_per_row - 1) * 6) as usize);

        const NUM_FLAT_ROWS: u32 = 25;
        const NUM_SMOOTHED_ROWS: u32 = 4;
        let slope = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);

        for z in 0..chunk_size {
            let total_z = start_index + z;
            for x in 0..self.vertices_per_row {
                let tx = (x as f32) / (self.vertices_per_row - 1) as f32 - 0.5;
                let x_position = tx * (self.vertices_per_row as f32) * self.quad_size.x;
                let z_position = (total_z as f32 - NUM_FLAT_ROWS as f32) * self.quad_size.y;

                if total_z < NUM_FLAT_ROWS {
                    positions.push([x_position, 0., z_position]);
                    normals.push(Vec3::Y.to_array());
                } else if total_z == NUM_FLAT_ROWS {
                    positions.push([x_position, -0.5, z_position]);
                    normals.push(Vec3::Y.to_array());
                } else {
                    let adjusted_z = total_z - NUM_FLAT_ROWS;
                    let noise_sample = noise.get([x as f64, adjusted_z as f64]) as f32;
                    let y_position = if adjusted_z < NUM_SMOOTHED_ROWS {
                        noise_sample * adjusted_z as f32 / NUM_SMOOTHED_ROWS as f32
                    } else {
                        noise_sample
                    };
                    let unsloped_position = Vec3::new(x_position, y_position, z_position);
                    let sloped_position = slope * unsloped_position;
                    positions.push(sloped_position.to_array());
                    normals.push(Vec3::new(0., 1., 1.).normalize().to_array());
                }
                // TODO: offsets for less repetitive uv?
                uvs.push([
                    tx,
                    (total_z % self.vertices_per_row) as f32 / self.vertices_per_row as f32,
                ]);
            }

            if z < chunk_size as u32 - 1 {
                for x in 0..(self.vertices_per_row - 1) {
                    let quad_index = self.vertices_per_row * z + x;
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

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
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
