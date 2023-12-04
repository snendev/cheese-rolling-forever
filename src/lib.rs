use noise::{NoiseFn, Perlin};

use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_xpbd_3d::prelude::*;

mod camera;
pub use camera::*;

mod cheese;
pub use cheese::*;

mod person;
pub use person::*;

mod systems;

pub struct CheeseGamePlugin;

impl Plugin for CheeseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_systems(Update, (systems::handle_inputs, systems::chase_cheese));
    }
}

pub struct RaceScenePlugin;

impl Plugin for RaceScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut images: ResMut<Assets<Image>>| {
                commands.spawn(DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        illuminance: 10.0e3,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                });
                commands.spawn(Cheese::bundle(&mut meshes, &mut materials));
                commands.spawn(Terrain::bundle(&mut meshes, &mut materials, &mut images));
            },
        );
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Terrain;

impl Terrain {
    const SIZE: f32 = 5000.;

    fn meshes() -> (Mesh, Collider) {
        let noise = Perlin::new(54321);
        let mesh: Mesh = shape::Plane {
            size: Self::SIZE,
            subdivisions: 1000,
        }
        .into();
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .map(|vertex_positions| {
                vertex_positions
                    .as_float3()
                    .unwrap()
                    .iter()
                    .map(|&[x, y, z]| [x, y + 1.5 * noise.get([x.into(), z.into()]) as f32, z])
                    .collect::<Vec<_>>()
            })
            .expect("mesh to have loaded some position buffer");
        let mesh = mesh.with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions);

        let collider = Collider::trimesh_from_mesh(&mesh).unwrap();
        (mesh, collider)
    }

    fn bundle(
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
                transform: Transform::default()
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_4)),
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
