use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{generate_terrain_noise, Cheese, Hand, Terrain};

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

pub(crate) fn update_terrain_mesh(
    mut commands: Commands,
    mut terrain_query: Query<(Entity, &mut Terrain, &Handle<Mesh>)>,
    cheese_query: Query<&Transform, With<Cheese>>,
    mut assets: ResMut<Assets<Mesh>>,
) {
    for (entity, mut terrain, handle) in terrain_query.iter_mut() {
        let Ok(cheese_transform) = cheese_query.get_single() else {
            continue;
        };
        let farthest_row_z = terrain.mesh_builder.quad_size.y * terrain.extents.1 as f32;
        let trigger_planar_distance = terrain.mesh_builder.quad_size.y * 100.;

        if (farthest_row_z - cheese_transform.translation.z).abs() >= trigger_planar_distance {
            continue;
        }

        let Some(mesh) = assets.get_mut(handle.id()) else {
            continue;
        };
        let noise = generate_terrain_noise();
        terrain.extend(20);
        *mesh = terrain.generate_mesh(&noise);

        // counts of the number of vertices and indices to remove
        commands
            .entity(entity)
            .remove::<Collider>()
            .insert(AsyncCollider(ComputedCollider::TriMesh));
    }
}
