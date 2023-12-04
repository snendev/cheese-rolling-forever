use bevy::prelude::*;

mod dolly;
use dolly::dolly::prelude::*;

use crate::Cheese;

#[derive(Clone, Copy, Debug, Default)]
#[derive(Component)]
pub struct PlayerCamera;

impl PlayerCamera {
    const RIG_Z_OFFSET: f32 = -2.;
    const RIG_Y_OFFSET: f32 = 5.;

    fn bundle(cheese_transform: &Transform) -> impl Bundle {
        let camera_offset = Vec3::new(0., Self::RIG_Y_OFFSET, Self::RIG_Z_OFFSET);
        let camera_translation = camera_offset + cheese_transform.translation;
        let target = cheese_transform.translation;
        (
            PlayerCamera,
            dolly::Rig::builder()
                .with(Position::new(cheese_transform.translation))
                .with(Smooth::new_position(1.))
                .with(Arm::new(camera_offset))
                .with(Smooth::new_rotation(1.))
                .with(LookAt::new(target))
                .build(),
            Camera3dBundle {
                transform: Transform::from_translation(camera_translation)
                    .looking_at(target, Vec3::Y),
                ..Default::default()
            },
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
        cheese_query: Query<(&Transform, &Cheese)>,
    ) {
        for mut rig in rig_query.iter_mut() {
            let (target, _cheese) = cheese_query.single();
            rig.driver_mut::<Position>().position = target.translation;
            let target = target.translation;
            rig.driver_mut::<LookAt>().target = target;
        }
    }
}

pub struct PlayerCameraPlugin;

impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(dolly::DollyPlugin::<PlayerCamera>::default())
            .add_systems(
                Update,
                (
                    PlayerCamera::spawn_for_added_cheese,
                    PlayerCamera::track_cheese,
                ),
            );
    }
}
