use bevy::prelude::*;

use crate::AppState;

mod ragdoll;
pub(crate) use ragdoll::*;

mod systems;
use systems::*;

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Person {
    size: f32,
    girth: f32,
}

impl Default for Person {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

impl Person {
    pub fn new(size: f32, girth: f32) -> Self {
        Self { size, girth }
    }
}

pub struct PersonPlugin;

impl Plugin for PersonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                chase_cheese,
                detect_grab,
                spawn_ragdolls,
                loop_ragdolls,
                despawn_infinites,
            )
                .run_if(in_state(AppState::Racing)),
        );
    }
}
