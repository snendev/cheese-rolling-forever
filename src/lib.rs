use bevy::prelude::*;

mod game;
pub use game::*;

mod menu;
pub use menu::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    SpawningScene,
    Countdown,
    Racing,
    Closing,
}
