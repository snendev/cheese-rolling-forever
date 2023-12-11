use bevy::prelude::*;

use crate::{Level, Terrain, TerrainNoise, TextureAssets};

pub(crate) fn update_terrain_mesh(
    mut commands: Commands,
    mut terrain_query: Query<(&mut Terrain, &Level)>,
    noise: Res<TerrainNoise>,
    textures: Res<TextureAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut terrain, level) in terrain_query.iter_mut() {
        terrain.update(
            level,
            &noise.get(),
            &mut commands,
            &textures,
            &mut meshes,
            &mut materials,
        );
    }
}
