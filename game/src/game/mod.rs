use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use crate::AppState;

mod camera;
pub use camera::*;

mod cheese;
pub use cheese::*;

mod person;
pub use person::*;

mod systems;

mod terrain;
pub use terrain::*;

pub struct CheeseRacePlugin;

impl Plugin for CheeseRacePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugins(PhysicsPlugins::default())
            .configure_sets(
                PostUpdate,
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .run_if(in_state(AppState::Racing).or_else(in_state(AppState::SpawningScene))),
            )
            .add_systems(
                Update,
                (
                    systems::handle_inputs,
                    systems::chase_cheese,
                    systems::detect_grab,
                    systems::spawn_ragdolls,
                    systems::loop_ragdolls,
                    systems::despawn_infinites,
                )
                    .run_if(in_state(AppState::Racing)),
            );
    }
}
