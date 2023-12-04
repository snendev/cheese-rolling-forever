use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{
        AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
    },
};

use bevy_geppetto::Test;

use bevy_xpbd_3d::{components::GravityScale, plugins::PhysicsDebugPlugin};

use cheese::{Cheese, CheeseGamePlugin, RaceScenePlugin};

fn main() {
    Test::new("Game scene".to_string(), |app| {
        app.add_plugins((
            CheeseGamePlugin,
            RaceScenePlugin,
            PhysicsDebugPlugin::default(),
            MaterialPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Update, (handle_start, scene_setup));
    })
    .run();
}

fn handle_start(inputs: Res<Input<KeyCode>>, mut q: Query<&mut GravityScale, With<Cheese>>) {
    if inputs.just_pressed(KeyCode::Space) {
        q.single_mut().0 = 1.;
    }
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 30.0,
            subdivisions: 100,
        })),
        transform: Transform::from_xyz(0., 0.5, 0.),
        material: materials.add(CustomMaterial {
            time: 0.,
            ship_position: Vec3 {
                x: 1.0,
                y: 3.0,
                z: 1.0,
            },
        }),
        // material: standard_materials.add(StandardMaterial {
        //     base_color: Color::BLUE,
        //     ..default()
        // }),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone, Asset, TypePath)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    ship_position: Vec3,
}

impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/example_shader.wgsl".into()
    }
}
