// globally useful systems

use bevy::prelude::*;

pub fn despawn_all_recursive<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
