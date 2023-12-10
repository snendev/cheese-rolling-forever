use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
};
use bevy_asset_loader::prelude::*;

use crate::{despawn_all_recursive, AppState};

pub struct CheeseAssetsPlugin {
    // allow tests to continue straight to other states
    pub after_load_state: AppState,
}

impl CheeseAssetsPlugin {
    pub fn new(after_load_state: AppState) -> Self {
        Self { after_load_state }
    }
}

impl Default for CheeseAssetsPlugin {
    fn default() -> Self {
        Self::new(AppState::Menu)
    }
}

impl Plugin for CheeseAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading).continue_to_state(self.after_load_state),
        )
        .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
        .add_systems(Startup, spawn_loading_ui)
        .add_systems(
            OnExit(AppState::Loading),
            (tile_terrain_assets, despawn_all_recursive::<LoadingUI>),
        )
        .add_systems(OnEnter(AppState::SpawningScene), play_bg_music);
    }
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/Dirt_Weeds01.jpg")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/Dirt_Weeds01_disp.jpg")]
    pub ground_displacement: Handle<Image>,
    #[asset(path = "textures/Dirt_Weeds01_normal.jpg")]
    pub ground_normal: Handle<Image>,
}

fn tile_terrain_assets(textures: Res<TextureAssets>, mut images: ResMut<Assets<Image>>) {
    for texture in [
        &textures.ground,
        &textures.ground_displacement,
        &textures.ground_normal,
    ] {
        if let Some(image) = images.get_mut(texture.clone()) {
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
                address_mode_u: ImageAddressMode::Repeat,
                address_mode_v: ImageAddressMode::Repeat,
                ..Default::default()
            });
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/CheeseOnTheMoon.wav")]
    pub bg_track: Handle<AudioSource>,
}

fn play_bg_music(mut commands: Commands, bg_track: Res<AudioAssets>) {
    commands.spawn(AudioBundle {
        source: bg_track.bg_track.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::Relative(VolumeLevel::new(0.5)),
            ..Default::default()
        },
    });
}

#[derive(Component)]
struct LoadingUI;

fn spawn_loading_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn((
            Name::new("Loading UI"),
            LoadingUI,
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font_size: 128.0,
                    color: Color::rgb(0.02, 0.02, 0.1),
                    ..Default::default()
                },
            ));
        });
}
