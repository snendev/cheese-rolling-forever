use bevy::prelude::*;

mod game;
pub use game::*;

mod menu;
pub use menu::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    #[default]
    Menu,
    Starting,
    Racing,
    Closing,
}
