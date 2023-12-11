use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::Cheese;

pub(crate) fn handle_inputs(
    inputs: Res<Input<KeyCode>>,
    mut query: Query<
        (
            &LinearVelocity,
            &mut ExternalImpulse,
            &mut ExternalAngularImpulse,
        ),
        With<Cheese>,
    >,
) {
    const INFLUENCE: f32 = 2.0;
    // "reference" refers to the reference frame, the coordinate system of the cheese's
    // downhill motion where "forward" is the direction of movement and "up" is perpendicular
    // to the hill.
    let reference_frame_influence = if inputs.pressed(KeyCode::Left) || inputs.pressed(KeyCode::A) {
        Some(-INFLUENCE)
    } else if inputs.pressed(KeyCode::Right) || inputs.pressed(KeyCode::D) {
        Some(INFLUENCE)
    } else {
        None
    };

    if let Some(influence) = reference_frame_influence {
        for (velocity, mut linear_impulse, mut angular_impulse) in query.iter_mut() {
            // weight shift along velocity axis
            let spin_axis = velocity.0.normalize();
            let torque_impulse = influence * spin_axis;
            let force_impulse = spin_axis.cross(Vec3::Y) * influence * 100.;
            if force_impulse.is_finite() {
                linear_impulse.set_impulse(force_impulse);
            }
            if torque_impulse.is_finite() {
                angular_impulse.set_impulse(torque_impulse);
            }
        }
    }
}
