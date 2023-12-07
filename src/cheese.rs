use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Cheese;

impl Cheese {
    // estimates at the size of the cheese wheel taken from
    // https://www.houseofcheese.co.uk/acatalog/A-Whole-Double-Gloucester-Cheese-25cm-dia-2310.html
    // 6cm height
    const HEIGHT: f32 = 0.4;
    // 12.5cm rad
    const RADIUS: f32 = 0.6;

    pub fn collider() -> Collider {
        Collider::cylinder(Self::HEIGHT, Self::RADIUS)
    }

    pub fn shape() -> shape::Cylinder {
        shape::Cylinder {
            height: Self::HEIGHT,
            radius: Self::RADIUS,
            ..Default::default()
        }
    }

    pub fn bundle(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> impl Bundle {
        (
            Cheese,
            Name::new("Cheese"),
            RigidBody::Dynamic,
            Self::collider(),
            ColliderDensity(100.),
            Restitution {
                coefficient: 0.0001,
                combine_rule: CoefficientCombine::Min,
            },
            Friction::new(0.5),
            LinearDamping(0.),
            AngularDamping(0.1),
            Dominance(1),
            PbrBundle {
                mesh: meshes.add(Self::shape().into()),
                material: materials.add(Color::rgb(1., 0.98, 0.8).into()),
                transform: Transform::from_translation(Vec3::Y * Self::RADIUS + 0.01)
                    .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2)),
                ..Default::default()
            },
        )
    }
}
