use bevy::prelude::*;

use bevy_geppetto::Test;

use bevy_xpbd_3d::{
    components::{AngularVelocity, LinearVelocity, Sleeping},
    plugins::PhysicsDebugPlugin,
};

use cheese::{Cheese, CheeseGamePlugin, Person, Terrain, TerrainNoise, TerrainPlugin};

fn main() {
    Test::new("Cheese controls".to_string(), |app| {
        app.add_plugins((
            CheeseGamePlugin,
            PhysicsDebugPlugin::default(),
            TerrainPlugin::default(),
        ))
        .insert_resource(TerrainNoise::from_noise(noise::Constant::new(0.)))
        .add_systems(Startup, handle_start);
    })
    .run();
}

fn handle_start(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2., 10., -6.).looking_at(Vec3::new(0., 5., -8.), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10.0e3,
            ..Default::default()
        },
        transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(Terrain::new((40, 40)).to_bundle());

    for (x, y) in (0..1).zip(0..1) {
        Person::default().spawn_ragdoll(
            Vec3::new(4. * x as f32, 5. + (4. * y as f32), -8. + (4. * y as f32)),
            &mut commands,
            &mut meshes,
            &mut materials,
        );
    }

    commands.spawn((Cheese, SpatialBundle::default()));
}

// TODO: strip lots away
// #[allow(clippy::type_complexity)]
// pub(crate) fn spawn_ragdolls(
//     mut commands: Commands,
//     ragdoll_query: Query<(Entity, &Transform), With<Person>>,
//     cheese_query: Query<&Transform, (With<Cheese>, Without<Person>)>,
//     time: Res<Time>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut last_spawned_time: Local<Duration>,
// ) {
//     let Ok(cheese_transform) = cheese_query.get_single() else {
//         return;
//     };

//     // how many ragdolls to keep active
//     const MAX_JUGGLE_COUNT: usize = 50;
//     const NEAR_MAX_COUNT: usize = 35;
//     // use different spawn rates when near max and not
//     const LOW_COUNT_SPAWN_RATE: Duration = Duration::from_secs(2);
//     const HIGH_COUNT_SPAWN_RATE: Duration = Duration::from_secs(4);

//     let time_since_last_spawn = time.elapsed() - *last_spawned_time;
//     let num_ragdolls = ragdoll_query.iter().count();

//     let mut rng = rand::thread_rng();
//     let mut spawn_ragdoll = |index: Option<usize>| {
//         info!(
//             "Spawning!! time: {:?} since last spawn {:?}",
//             time.elapsed(),
//             time_since_last_spawn
//         );
//         let index = index.unwrap_or_else(|| rng.gen_range(0..5));
//         Person::new(
//             0.5 + rng.gen_range(1..=10) as f32 / 10.,
//             0.5 + rng.gen_range(1..=10) as f32 / 10.,
//         )
//         .spawn_ragdoll(
//             get_spawn_point(&mut rng, cheese_transform.translation, index),
//             &mut commands,
//             &mut meshes,
//             &mut materials,
//         );
//         *last_spawned_time = time.elapsed();
//     };

//     if num_ragdolls > MAX_JUGGLE_COUNT {
//         // must have goofed somewhere
//         // let ragdolls_to_delete = ragdoll_query.iter()
//         // commands.entity(entity).despawn();
//     } else if num_ragdolls == MAX_JUGGLE_COUNT {
//         // do nothing
//     } else if num_ragdolls > NEAR_MAX_COUNT {
//         // spawn ragdolls slowly
//         if time_since_last_spawn > HIGH_COUNT_SPAWN_RATE {
//             spawn_ragdoll(None)
//         }
//     } else {
//         // spawn bursts of ragdolls
//         if time_since_last_spawn > LOW_COUNT_SPAWN_RATE {
//             // for index in 0..5 {
//             spawn_ragdoll(None);
//             // }
//         }
//     }
// }
