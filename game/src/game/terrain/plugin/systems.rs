use bevy::prelude::*;

use crate::{Cheese, Terrain, TerrainNoise, TextureAssets};

pub(crate) fn update_terrain_mesh(
    mut commands: Commands,
    mut terrain_query: Query<&mut Terrain>,
    cheese_query: Query<&Transform, With<Cheese>>,
    noise: Res<TerrainNoise>,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(mut terrain) = terrain_query.get_single_mut() else {
        return;
    };
    let Ok(cheese_transform) = cheese_query.get_single() else {
        return;
    };

    terrain.update(
        cheese_transform.translation,
        &noise.get(),
        &mut commands,
        &textures,
        &mut meshes,
        &mut materials,
    );
}
