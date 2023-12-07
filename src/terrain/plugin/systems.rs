use bevy::prelude::*;

use crate::{Cheese, Terrain, TerrainNoise};

pub(crate) fn update_terrain_mesh(
    mut commands: Commands,
    mut terrain_query: Query<&mut Terrain>,
    cheese_query: Query<&Transform, With<Cheese>>,
    noise: Res<TerrainNoise>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
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
        &mut meshes,
        &mut materials,
        &mut images,
    );
}
