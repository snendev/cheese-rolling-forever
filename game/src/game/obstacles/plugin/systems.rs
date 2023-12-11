use bevy::prelude::*;

use crate::{Level, ObstacleNoise, Obstacles, TextureAssets};

pub(super) fn seed_noise(mut commands: Commands) {
    commands.insert_resource(ObstacleNoise::new(rand::random()));
}

pub(super) fn attach_obstacles(mut commands: Commands, query: Query<Entity, Added<Level>>) {
    for entity in query.iter() {
        commands.entity(entity).insert(Obstacles::default());
    }
}

pub(super) fn update_obstacles(
    mut commands: Commands,
    mut obstacles_query: Query<(&mut Obstacles, &Level)>,
    noise: Res<ObstacleNoise>,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut obstacles, level) in obstacles_query.iter_mut() {
        obstacles.update(
            level,
            &noise.get(),
            &mut commands,
            &textures,
            &mut meshes,
            &mut materials,
        );
    }
}
