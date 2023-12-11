use rand::Rng;
use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};

use bevy_xpbd_3d::prelude::*;

use crate::{AppState, Cheese, Hand, Person};

// systems
const CHEESE_PULL_STRENGTH: f32 = 1e6;
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
    hand_query: Query<&CollidingEntities, With<Person>>,
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

// aka the "lakitu" system
pub(crate) fn loop_ragdolls(
    mut ragdoll_query: Query<
        (&mut Transform, &mut LinearVelocity, &mut AngularVelocity),
        With<Person>,
    >,
    cheese_query: Query<(&Transform, &LinearVelocity), (With<Cheese>, Without<Person>)>,
) {
    let Ok((cheese_transform, cheese_velocity)) = cheese_query.get_single() else {
        return;
    };

    // let mut rng = rand::thread_rng();
    let mut count_looped_this_frame = 0;
    // let random_offset = rng.gen_range(0..5);
    for (mut transform, mut linvel, mut angvel) in ragdoll_query.iter_mut() {
        if (cheese_transform.translation.y - transform.translation.y).abs() >= 300.
            || transform.translation.is_nan()
        {
            *transform = Transform::from_translation(get_spawn_point(
                cheese_transform.translation,
                count_looped_this_frame, // + random_offset
                0.,
            ));
            count_looped_this_frame += 1;
            *linvel = cheese_velocity.0.into();
            *angvel = AngularVelocity::ZERO;
        }
    }
}

pub(crate) fn spawn_ragdolls(
    mut commands: Commands,
    ragdoll_query: Query<(Entity, &Transform), With<Person>>,
    cheese_query: Query<(&Transform, &LinearVelocity), (With<Cheese>, Without<Person>)>,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut last_spawned_time: Local<Duration>,
) {
    let Ok((cheese_transform, cheese_velocity)) = cheese_query.get_single() else {
        return;
    };

    // how many ragdolls to keep active
    const MAX_JUGGLE_COUNT: usize = 50;
    const NEAR_MAX_COUNT: usize = 35;
    // use different spawn rates when near max and not
    const LOW_COUNT_SPAWN_RATE: Duration = Duration::from_secs(1);
    const HIGH_COUNT_SPAWN_RATE: Duration = Duration::from_secs(4);

    let time_since_last_spawn = time.elapsed() - *last_spawned_time;
    let num_ragdolls = ragdoll_query.iter().count();

    let mut rng = rand::thread_rng();
    let mut spawn_ragdoll = |index: Option<usize>| {
        let index = index.unwrap_or_else(|| rng.gen_range(0..5));
        Person::new(
            1.5 + rng.gen_range(1..=10) as f32 / 5.,
            1.5 + rng.gen_range(1..=10) as f32 / 5.,
        )
        .spawn_ragdoll(
            get_spawn_point(cheese_transform.translation, index, 0.),
            cheese_velocity.0,
            &mut commands,
            &mut meshes,
            &mut materials,
        );
        *last_spawned_time = time.elapsed();
    };

    if num_ragdolls > MAX_JUGGLE_COUNT {
        // must have goofed somewhere
        // let ragdolls_to_delete = ragdoll_query.iter()
        // commands.entity(entity).despawn();
    } else if num_ragdolls == MAX_JUGGLE_COUNT {
        // do nothing
    } else if num_ragdolls > NEAR_MAX_COUNT {
        // spawn ragdolls slowly
        if time_since_last_spawn > HIGH_COUNT_SPAWN_RATE {
            spawn_ragdoll(None)
        }
    } else {
        // spawn bursts of ragdolls
        if time_since_last_spawn > LOW_COUNT_SPAWN_RATE {
            for index in 0..2 {
                spawn_ragdoll(Some(index + 1));
            }
        }
    }
}

// add random x later
const LAKITU_OFFSET: Vec3 = Vec3::new(0., 60., -40.);

fn get_spawn_point(cheese_translation: Vec3, index: usize, additional_offset: f32) -> Vec3 {
    const AVG_GAP: f32 = 8.;
    cheese_translation
        + LAKITU_OFFSET
        + Vec3::X * AVG_GAP * (2. - index as f32)
        + Vec3::X * additional_offset
}
