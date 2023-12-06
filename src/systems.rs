use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::{Cheese, Hand, Terrain};

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
    const INFLUENCE: f32 = 2.0e-2;
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
            angular_impulse.set_impulse(torque_impulse);
            linear_impulse.set_impulse(spin_axis.cross(Vec3::Y) * influence * 100.);
        }
    }
}

pub(crate) fn update_terrain_mesh(
    mut commands: Commands,
    mut terrain_query: Query<(Entity, &mut Terrain, &Handle<Mesh>)>,
    cheese_query: Query<&Transform, With<Cheese>>,
    spatial_query: SpatialQuery,
    mut assets: ResMut<Assets<Mesh>>,
) {
    let Ok((entity, mut terrain, handle)) = terrain_query.get_single_mut() else {
        return;
    };
    let Ok(cheese_transform) = cheese_query.get_single() else {
        return;
    };
    let Some(mesh) = assets.get_mut(handle.id()) else {
        return;
    };

    let farthest_row_z = terrain.mesh_builder.quad_size.y * terrain.extents.1 as f32;
    let trigger_planar_distance = terrain.mesh_builder.quad_size.y * 180.;

    if (farthest_row_z - cheese_transform.translation.z).abs() >= trigger_planar_distance {
        return;
    }

    terrain.extend(20);
    *mesh = terrain.generate_mesh(&terrain.generate_noise());

    for entity in spatial_query.shape_intersections(
        &Collider::trimesh_from_mesh(&mesh).unwrap(),
        Vec3::ZERO,
        Quat::default(),
        SpatialQueryFilter::default(),
    ) {
        info!("{:?}", entity);
    }

    // counts of the number of vertices and indices to remove
    commands
        .entity(entity)
        .remove::<Collider>()
        .insert(AsyncCollider(ComputedCollider::TriMesh));
}

// pub(crate) fn lakitu_system(
//     mut person_query: Query<Person,
// ) {

// }
