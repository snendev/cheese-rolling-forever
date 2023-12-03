use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use bevy_geppetto::Test;

use bevy_xpbd_3d::{
    components::{Collider, GravityScale},
    plugins::PhysicsDebugPlugin,
    resources::Gravity,
};

use cheese::{Cheese, CheeseGamePlugin, Person};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct FakeSchedule;

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.insert_resource(Gravity(Vec3::ZERO))
            .add_plugins((CheeseGamePlugin, PhysicsDebugPlugin::default()))
            .add_systems(Update, (handle_start, remove_gravity_scale))
            .add_systems(Startup, spawn_scene);
    })
    .run();
}

fn remove_gravity_scale(mut commands: Commands, mut q: Query<Entity, Added<Collider>>) {
    for entity in q.iter_mut() {
        commands.entity(entity).insert(GravityScale(0.));
    }
}

fn handle_start(inputs: Res<Input<KeyCode>>, mut q: Query<&mut GravityScale>) {
    if inputs.just_pressed(KeyCode::Space) {
        for mut gravity in q.iter_mut() {
            gravity.0 = 1.;
        }
    }
}

fn spawn_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 5.).looking_at(Vec3::ZERO, Vec3::Y),
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

    Person::default().spawn_ragdoll(
        Vec3::new(3., 0., -3.),
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    commands.spawn(Cheese::bundle(&mut meshes, &mut materials));
}
