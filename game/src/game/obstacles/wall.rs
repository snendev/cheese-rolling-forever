use bevy_xpbd_3d::components::{Collider, ColliderDensity, RigidBody};

use bevy::prelude::*;

use crate::{Chunk, Vertex};

#[derive(Clone, Debug)]
#[derive(Component, Reflect)]
pub struct Wall {
    pub size: Vec2,
    pub chunk: Chunk,
    pub vertex: Vertex,
}

impl Wall {
    const HEIGHT: f32 = 40.;

    pub fn new(chunk: Chunk, vertex: Vertex, size: Vec2) -> Self {
        Self {
            chunk,
            vertex,
            size,
        }
    }

    pub fn to_bundle(
        self,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> impl Bundle {
        let global_vertex = self.chunk.to_global_coords(self.vertex);
        let grid_position = self.chunk.to_translation(global_vertex);
        let sloped_translation = Vec3::new(
            grid_position.x + self.size.x / 2.,
            -grid_position.y + Self::HEIGHT * 0.45,
            grid_position.y,
        );
        (
            Name::new(format!("Wall ({},{})", global_vertex.x, global_vertex.z)),
            RigidBody::Static,
            Collider::cuboid(self.size.x, Self::HEIGHT, self.size.y),
            ColliderDensity(1.),
            PbrBundle {
                mesh: meshes.add(shape::Box::new(self.size.x, Self::HEIGHT, self.size.y).into()),
                material: materials.add(Color::RED.into()),
                transform: Transform::from_translation(sloped_translation)
                    .with_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_8)),
                ..Default::default()
            },
            self,
        )
    }
}
