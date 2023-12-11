use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{AppState, SceneAssets};

mod systems;
use systems::*;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Cheese;

impl Cheese {
    // estimates at the size of the cheese wheel taken from
    // https://www.houseofcheese.co.uk/acatalog/A-Whole-Double-Gloucester-Cheese-25cm-dia-2310.html
    // 6cm height
    pub const HEIGHT: f32 = 0.4;
    // 12.5cm rad
    pub const RADIUS: f32 = 0.6;

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

    pub fn default_transform() -> Transform {
        Transform::from_translation(Vec3::Y * Self::RADIUS * 4.)
            .with_rotation(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2))
    }

    pub fn graphic(transform: Transform, scenes: &SceneAssets) -> SceneBundle {
        SceneBundle {
            // TODO: Why does this often fail to pull the correct asset
            scene: scenes.cheese_good.clone(),
            transform,
            ..Default::default()
        }
    }

    pub fn bundle(transform: Transform, scenes: &SceneAssets) -> impl Bundle {
        (
            Cheese,
            Name::new("Cheese"),
            RigidBody::Dynamic,
            Self::collider(),
            ColliderDensity(900.),
            Restitution {
                coefficient: 0.0001,
                combine_rule: CoefficientCombine::Min,
            },
            Friction::new(0.5),
            LinearDamping(0.08),
            AngularDamping(0.08),
            Dominance(1),
            Self::graphic(transform, scenes),
        )
    }
}

pub struct CheesePlugin;

impl Plugin for CheesePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_inputs.run_if(in_state(AppState::Racing)));
    }
}
