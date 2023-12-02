use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod camera;
use camera::PlayerCameraPlugin;

pub struct CheeseGamePlugin;

impl Plugin for CheeseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhysicsPlugins::default(), PlayerCameraPlugin));
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
                commands.spawn(PointLightBundle {
                    point_light: PointLight::default(),
                    transform: Transform::from_xyz(0., 8., 5.),
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
pub struct Cheese;

impl Cheese {
    // estimates at the size of the cheese wheel taken from
    // https://www.houseofcheese.co.uk/acatalog/A-Whole-Double-Gloucester-Cheese-25cm-dia-2310.html
    // 6cm height
    const HEIGHT: f32 = 0.06;
    // 12.5cm rad
    const RADIUS: f32 = 0.125;

    fn collider() -> Collider {
        Collider::cylinder(Self::HEIGHT, Self::RADIUS)
    }

    fn shape() -> shape::Cylinder {
        shape::Cylinder {
            height: Self::HEIGHT,
            radius: Self::RADIUS,
            ..Default::default()
        }
    }

    fn bundle(meshes: &mut Assets<Mesh>, materials: &mut Assets<StandardMaterial>) -> impl Bundle {
        (
            Cheese,
            Name::new("Cheese"),
            RigidBody::Dynamic,
            Self::collider(),
            GravityScale(0.),
            PbrBundle {
                mesh: meshes.add(Self::shape().into()),
                material: materials.add(Color::rgb(1., 0.98, 0.8).into()),
                transform: Transform::from_translation(Vec3::Y * 1.)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..Default::default()
            },
        )
    }
}

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Terrain;

impl Terrain {
    const SIZE: f32 = 100.;

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
