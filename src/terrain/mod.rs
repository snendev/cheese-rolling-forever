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
use noise_utils::generate_terrain_noise;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Terrain;

impl Terrain {
    const SIZE: f32 = 500.;

    pub fn meshes() -> (Mesh, Collider) {
        let noise = generate_terrain_noise();
        let mesh: Mesh = Rectangle {
            size: Vec2::new(30., Self::SIZE),
            subdivision_size: Vec2::new(1., 1.),
        }
        .into();
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .map(|vertex_positions| {
                vertex_positions
                    .as_float3()
                    .unwrap()
                    .iter()
                    .map(|&[x, y, z]| {
                        [
                            x,
                            y + noise.get([(x / 10.).into(), (z / 10.).into()]) as f32,
                            z,
                        ]
                    })
                    .collect::<Vec<_>>()
            })
            .expect("mesh to have loaded some position buffer");
        let mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        let collider = Collider::trimesh_from_mesh(&mesh).unwrap();
        (mesh, collider)
    }

    pub fn bundle(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        images: &mut Assets<Image>,
    ) -> impl Bundle {
        let (mesh, collider) = Self::meshes();
        (
            Terrain,
            Name::new("Terrain"),
            RigidBody::Static,
            collider,
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(StandardMaterial {
                    base_color_texture: Some(images.add(uv_debug_texture())),
                    ..default()
                }),
                transform: Transform::from_translation(Vec3::new(
                    0.,
                    -Self::SIZE / 4.,
                    -Self::SIZE / 4.,
                ))
                .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
                ..Default::default()
            },
        )
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

/// A rectangle on the `XY` plane centered at the origin.
/// Adapted from shape::Quad and shape::Plane to have a subdivided Quad
#[derive(Debug, Copy, Clone)]
pub struct Rectangle {
    /// Full width and height of the rectangle.
    pub size: Vec2,
    /// the size of each "chunk" of vertices on the mesh
    pub subdivision_size: Vec2,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle::new(Vec2::ONE, Vec2::ONE)
    }
}

impl Rectangle {
    pub fn new(size: Vec2, subdivision_size: Vec2) -> Self {
        Self {
            size,
            subdivision_size,
        }
    }
}

impl From<Rectangle> for Mesh {
    fn from(rectangle: Rectangle) -> Self {
        let Vec2 {
            x: x_vertex_count,
            y: z_vertex_count,
        } = (rectangle.size * 2. / rectangle.subdivision_size).floor();
        let x_vertex_count = x_vertex_count as u32;
        let z_vertex_count = z_vertex_count as u32;

        let num_vertices = (z_vertex_count * x_vertex_count) as usize;
        let num_indices = (z_vertex_count - 1) * (x_vertex_count - 1) * 6;
        let up = Vec3::Y.to_array();

        let mut positions: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(num_vertices);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(num_vertices);
        let mut indices: Vec<u32> = Vec::with_capacity(num_indices as usize);

        for z in 0..z_vertex_count {
            for x in 0..x_vertex_count {
                // just use subdivision_size here?
                let tx = x as f32 / (x_vertex_count - 1) as f32;
                let tz = z as f32 / (z_vertex_count - 1) as f32;
                positions.push([
                    (-0.5 + tx) * rectangle.size.x,
                    0.0,
                    (-0.5 + tz) * rectangle.size.y,
                ]);
                normals.push(up);
                uvs.push([tx, tz]);
            }
        }

        for y in 0..z_vertex_count - 1 {
            for x in 0..x_vertex_count - 1 {
                let rectangle = y * x_vertex_count + x;
                indices.push(rectangle + x_vertex_count + 1);
                indices.push(rectangle + 1);
                indices.push(rectangle + x_vertex_count);
                indices.push(rectangle);
                indices.push(rectangle + x_vertex_count);
                indices.push(rectangle + 1);
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}
