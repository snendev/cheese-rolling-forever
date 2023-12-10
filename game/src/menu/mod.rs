use bevy::prelude::*;

use crate::{despawn_all_recursive, AppState};

mod graphic;
use graphic::*;

mod ui;
use ui::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::Menu),
            (spawn_menu_camera, spawn_start_menu, spawn_graphic),
        )
        .add_systems(
            OnExit(AppState::Menu),
            (
                despawn_all_recursive::<MenuCamera>,
                despawn_all_recursive::<MenuUI>,
                despawn_all_recursive::<MenuGraphic>,
            ),
        )
        .add_systems(Update, (spin_graphic, handle_play));
    }
}

#[derive(Component)]
pub(super) struct MenuCamera;

fn spawn_menu_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Menu Camera"),
        MenuCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0., 5., 10.).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    ));
}
