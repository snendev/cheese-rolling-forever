use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use bevy_geppetto::Test;

use bevy_xpbd_3d::{
    components::{Collider, GravityScale},
    plugins::{PhysicsDebugPlugin, PhysicsPlugins},
};

use cheese::{Person, RaceScenePlugin};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ScheduleLabel)]
pub struct FakeSchedule;

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            PhysicsPlugins::default(),
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
        ))
        .add_systems(Update, (handle_start, remove_gravity_scale))
        .add_systems(Startup, (spawn_person, spawn_camera));
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

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 3., 5.).looking_at(Vec3::Y * 2., Vec3::Y),
        ..Default::default()
    });
}

fn spawn_person(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    Person::default().spawn_ragdoll(Vec3::Y * 2., &mut commands, &mut meshes, &mut materials);
}
