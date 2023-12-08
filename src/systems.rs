use bevy::{prelude::*, utils::HashSet};
use bevy_xpbd_3d::prelude::*;

use crate::{AppState, Cheese, Hand, Person};

// systems
const CHEESE_PULL_STRENGTH: f32 = 10.0;
pub(crate) fn chase_cheese(
    mut arm_query: Query<(&Transform, &mut ExternalImpulse), With<Hand>>,
    cheese_query: Query<&Transform, With<Cheese>>,
) {
    let Ok(cheese_transform) = cheese_query.get_single() else {
        return;
    };
    arm_query.par_iter_mut().for_each(|(transform, mut force)| {
        // each tick arms receive a magnetic impulse towards the cheese
        let delta = cheese_transform.translation - transform.translation;
        // N.B. this overwrites
        force.set_impulse(CHEESE_PULL_STRENGTH * delta / delta.length_squared());
    });
}

pub(crate) fn detect_grab(
    hand_query: Query<&CollidingEntities, With<Hand>>,
    cheese_query: Query<&Cheese>,
    mut state: ResMut<NextState<AppState>>,
) {
    for colliding_entities in hand_query.iter() {
        for entity in colliding_entities.0.iter() {
            if cheese_query.contains(*entity) {
                info!("Caught the cheese!!!!");
                state.set(AppState::Closing);
            }
        }
    }
}

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
    let reference_frame_influence = if inputs.pressed(KeyCode::Left) {
        Some(-INFLUENCE)
    } else if inputs.pressed(KeyCode::Right) {
        Some(INFLUENCE)
    } else {
        None
    };

    if let Some(influence) = reference_frame_influence {
        for (velocity, mut linear_impulse, mut angular_impulse) in query.iter_mut() {
            // weight shift along velocity axis
            let spin_axis = velocity.0.normalize();
            let torque_impulse = influence * spin_axis;
            if torque_impulse.is_finite() {
                angular_impulse.set_impulse(torque_impulse);
                linear_impulse.set_impulse(spin_axis.cross(Vec3::Y) * influence * 100.);
            }
        }
    }
}

// aka the "lakitu" system
pub(crate) fn loop_ragdolls(
    mut ragdoll_query: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<Person>,
    >,
    cheese_query: Query<&Transform, (With<Cheese>, Without<Person>)>,
) {
    let Ok(cheese_transform) = cheese_query.get_single() else {
        return;
    };

    // let mut rng = rand::thread_rng();
    let mut count_looped_this_frame = 0;
    // let random_offset = rng.gen_range(0..5);
    for (mut transform, mut linvel, mut angvel) in ragdoll_query.iter_mut() {
        if (cheese_transform.translation.y - transform.translation.y).abs() >= 300.
            || transform.translation.is_nan()
        {
            info!(
                "Lakitu! {} - {}",
                cheese_transform.translation.y, transform.translation.y
            );
            *transform = Transform::from_translation(get_spawn_point(
                cheese_transform.translation,
                count_looped_this_frame, // + random_offset
                0.,
            ));
            count_looped_this_frame += 1;
            *linvel = LinearVelocity::ZERO;
            *angvel = AngularVelocity::ZERO;
        }
    }
}

pub(crate) fn despawn_infinites(
    mut commands: Commands,
    query: Query<(
        Entity,
        &Transform,
        &LinearVelocity,
        &AngularVelocity,
        Option<&Parent>,
    )>,
) {
    let entities_to_remove = query
        .iter()
        .filter_map(|(entity, transform, linvel, angvel, parent)| {
            if !transform.translation.is_finite()
                || !transform.rotation.is_finite()
                || !linvel.0.is_finite()
                || !angvel.0.is_finite()
            {
                if let Some(parent) = parent {
                    Some(parent.get())
                } else {
                    Some(entity)
                }
            } else {
                None
            }
        })
        .collect::<HashSet<Entity>>();
    for entity in entities_to_remove {
        commands.entity(entity).despawn_recursive();
    }
}

// add random x later
const LAKITU_OFFSET: Vec3 = Vec3::new(0., 6., 5.);

fn get_spawn_point(cheese_translation: Vec3, index: usize, additional_offset: f32) -> Vec3 {
    const AVG_GAP: f32 = 8.;
    cheese_translation
        + LAKITU_OFFSET
        + Vec3::X * AVG_GAP * (2. - index as f32)
        + Vec3::X * additional_offset
}
