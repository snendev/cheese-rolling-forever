use bevy::prelude::*;
use bevy_atmosphere::plugin::{AtmosphereCamera, AtmospherePlugin};
use bevy_xpbd_3d::components::LinearVelocity;

mod dolly;
use dolly::dolly::prelude::*;

use crate::Cheese;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
pub struct PlayerCamera;

impl PlayerCamera {
    const RIG_Z_OFFSET: f32 = -3.;
    const RIG_Y_OFFSET: f32 = 4.;
    const CAM_OFFSET: Vec3 = Vec3::new(0., Self::RIG_Y_OFFSET, Self::RIG_Z_OFFSET);

    fn bundle(cheese_transform: &Transform) -> impl Bundle {
        let camera_translation = Self::CAM_OFFSET + cheese_transform.translation;
        let target = cheese_transform.translation;
        (
            PlayerCamera,
            dolly::Rig::builder()
                .with(Position::new(cheese_transform.translation))
                .with(Smooth::new_position(1.))
                .with(Arm::new(Self::CAM_OFFSET))
                .with(LookAt::new(target))
                .with(Smooth::new_rotation(1.))
                .build(),
            Camera3dBundle {
                transform: Transform::from_translation(camera_translation)
                    .looking_at(target, Vec3::Y),
                ..Default::default()
            },
            AtmosphereCamera::default(),
        )
    }

    fn spawn_for_added_cheese(
        mut commands: Commands,
        cheese_query: Query<&Transform, Added<Cheese>>,
    ) {
        if let Ok(transform) = cheese_query.get_single() {
            commands.spawn(Self::bundle(transform));
        }
    }

    fn track_cheese(
        mut rig_query: Query<&mut dolly::Rig, With<PlayerCamera>>,
        cheese_query: Query<(&Transform, &LinearVelocity, &Cheese)>,
        camera_direction: Res<CameraDirection>,
    ) {
        for mut rig in rig_query.iter_mut() {
            let (target, velocity, _cheese) = cheese_query.single();
            // if cheese goes into oblivion, at least don't make the camera follow it there
            if !target.translation.is_finite() {
                continue;
            }

            rig.driver_mut::<Position>().position = target.translation;

            rig.driver_mut::<Arm>().offset = match *camera_direction {
                CameraDirection::Forward => Self::CAM_OFFSET,
                CameraDirection::Backward => Vec3::new(
                    Self::CAM_OFFSET.x,
                    Self::CAM_OFFSET.z * 0.5 - velocity.z * 0.5,
                    Self::CAM_OFFSET.y * 2. + velocity.z * 0.5,
                ),
            };
            let target = target.translation;
            rig.driver_mut::<LookAt>().target = target;
        }
    }

    fn look_behind_input(
        inputs: Res<Input<KeyCode>>,
        mut camera_direction: ResMut<CameraDirection>,
    ) {
        if inputs.pressed(KeyCode::Space) {
            *camera_direction = CameraDirection::Backward;
        } else {
            *camera_direction = CameraDirection::Forward;
        }
    }
}

#[derive(Default, Resource)]
enum CameraDirection {
    #[default]
    Forward,
    Backward,
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraDirection>()
            .add_plugins((
                dolly::DollyPlugin::<PlayerCamera>::default(),
                AtmospherePlugin,
            ))
            .add_systems(
                Update,
                (
                    PlayerCamera::spawn_for_added_cheese,
                    PlayerCamera::track_cheese,
                    PlayerCamera::look_behind_input,
                ),
            );
    }
}
