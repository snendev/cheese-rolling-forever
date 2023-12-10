use bevy::prelude::*;

mod assets;
pub use assets::*;

mod game;
pub use game::*;

mod menu;
pub use menu::*;

mod systems;
pub use systems::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    Menu,
    SpawningScene,
    Countdown,
    Racing,
    Closing,
}
