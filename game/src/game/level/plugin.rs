use bevy::prelude::*;

use crate::{Cheese, Chunk, Level, Vertex};

#[derive(Debug, Default)]
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Vertex>()
            .register_type::<Chunk>()
            .add_systems(Update, update_level);
    }
}

fn update_level(mut level_query: Query<&mut Level>, cheese_query: Query<&Transform, With<Cheese>>) {
    let Ok(mut level) = level_query.get_single_mut() else {
        return;
    };
    let Ok(cheese_transform) = cheese_query.get_single() else {
        return;
    };

    level.update(cheese_transform.translation);
}
