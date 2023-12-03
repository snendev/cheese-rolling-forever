use bevy::prelude::*;
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
             mut materials: ResMut<Assets<StandardMaterial>>| {
                commands.spawn(DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        illuminance: 10.0e3,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                });
                commands.spawn(Cheese::bundle(&mut meshes, &mut materials));
                commands.spawn(Terrain::bundle(&mut meshes, &mut materials));
            },
        );
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Terrain;

impl Terrain {
    const SIZE: f32 = 1000.;

    fn meshes() -> (Mesh, Collider) {
        let mesh = shape::Plane {
            size: Self::SIZE,
            subdivisions: 10,
        }
        .into();
        let collider = Collider::trimesh_from_mesh(&mesh).unwrap();
        (mesh, collider)
    }

    fn bundle(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
        let (mesh, collider) = Self::meshes();
        (
            Terrain,
            Name::new("Terrain"),
            RigidBody::Static,
            collider,
            PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::BEIGE.into()),
                transform: Transform::default()
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_6)),
                ..Default::default()
            },
        )
    }
}
