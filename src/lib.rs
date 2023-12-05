use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod camera;
pub use camera::*;

mod cheese;
pub use cheese::*;

mod person;
pub use person::*;

mod terrain;
pub use terrain::*;

mod systems;

pub struct CheeseGamePlugin;

impl Plugin for CheeseGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default()).add_systems(
            Update,
            (
                systems::handle_inputs,
                systems::chase_cheese,
                systems::update_terrain_mesh,
            ),
        );
    }
}

pub struct RaceScenePlugin;

impl Plugin for RaceScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            |mut commands: Commands,
             mut meshes: ResMut<Assets<Mesh>>,
             mut materials: ResMut<Assets<StandardMaterial>>,
             mut images: ResMut<Assets<Image>>| {
                commands.spawn(DirectionalLightBundle {
                    directional_light: DirectionalLight {
                        illuminance: 10.0e3,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(2., 10., 5.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                });
                commands.spawn(Cheese::bundle(&mut meshes, &mut materials));
                commands.spawn(Terrain::bundle(&mut meshes, &mut materials, &mut images));
            },
        );
    }
}
