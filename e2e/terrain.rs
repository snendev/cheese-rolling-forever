use bevy::{pbr::wireframe::WireframePlugin, prelude::*};

use bevy_geppetto::Test;

use bevy_xpbd_3d::{
    components::{AsyncCollider, Collider, ComputedCollider},
    plugins::{PhysicsDebugPlugin, PhysicsPlugins},
};

use cheese::{generate_terrain_noise, Cheese, RaceScenePlugin, Terrain};

fn main() {
    Test::new("Terrain alone".to_string(), |app| {
        app.add_plugins((
            PhysicsPlugins::default(),
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
            WireframePlugin::default(),
        ))
        .add_systems(Startup, spawn_scene)
        .add_systems(Update, update_terrain_mesh);
    })
    .run();
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 5., -2.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn update_terrain_mesh(
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
