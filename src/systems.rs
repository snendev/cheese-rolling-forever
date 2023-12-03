use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{Cheese, Hand};

// systems
const CHEESE_PULL_STRENGTH: f32 = 10.0;
pub(crate) fn chase_cheese(
    mut arm_query: Query<(&Transform, &mut ExternalImpulse), With<Hand>>,
    cheese_query: Query<&Transform, With<Cheese>>,
) {
    let Ok(cheese_transform) = cheese_query.get_single() else {
        return;
    };
    for (transform, mut force) in arm_query.iter_mut() {
        // each tick arms receive a magnetic impulse towards the cheese
        let delta = cheese_transform.translation - transform.translation;
        // N.B. this overwrites
        force.set_impulse(CHEESE_PULL_STRENGTH * delta / delta.length_squared());
    }
}

pub(crate) fn handle_inputs(
    inputs: Res<Input<KeyCode>>,
    mut query: Query<(&LinearVelocity, &mut ExternalAngularImpulse), With<Cheese>>,
) {
    const INFLUENCE: f32 = 1.0e-2;
    // "reference" refers to the reference frame, the coordinate system of the cheese's
    // downhill motion where "forward" is the direction of movement and "up" is perpendicular
    // to the hill.
    let reference_frame_influence = if inputs.pressed(KeyCode::Left) {
        Some(-INFLUENCE)
    } else if inputs.pressed(KeyCode::Right) {
        Some(INFLUENCE)
    } else {
        None
    };

    if let Some(influence) = reference_frame_influence {
        for (velocity, mut impulse) in query.iter_mut() {
            // weight shift along velocity axis
            let spin_axis = velocity.0.normalize();
            let torque_impulse = influence * spin_axis;
            impulse.set_impulse(torque_impulse);
        }
    }
}