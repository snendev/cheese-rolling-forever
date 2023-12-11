use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
};

use bevy_asset_loader::prelude::*;
use bevy_kira_audio::{
    prelude::{Audio, AudioPlugin, AudioSource},
    AudioControl,
};

use crate::{despawn_all_recursive, AppState};

pub struct SceneAssetsPlugin {
    // allow tests to continue straight to other states
    pub after_load_state: AppState,
}

impl SceneAssetsPlugin {
    pub fn new(after_load_state: AppState) -> Self {
        Self { after_load_state }
    }
}

impl Default for SceneAssetsPlugin {
    fn default() -> Self {
        Self::new(AppState::Menu)
    }
}

impl Plugin for SceneAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_loading_state(
                LoadingState::new(AppState::Loading).continue_to_state(self.after_load_state),
            )
            .add_collection_to_loading_state::<_, FontAssets>(AppState::Loading)
            .add_collection_to_loading_state::<_, SceneAssets>(AppState::Loading)
            .add_collection_to_loading_state::<_, TextureAssets>(AppState::Loading)
            .add_collection_to_loading_state::<_, AudioAssets>(AppState::Loading)
            .add_systems(Startup, spawn_loading_ui)
            .add_systems(
                OnExit(AppState::Loading),
                (
                    tile_terrain_assets,
                    despawn_all_recursive::<LoadingUI>,
                    despawn_all_recursive::<LoadingUICamera>,
                ),
            )
            .add_systems(
                OnEnter(AppState::SpawningScene),
                // after we play once, just keep the loop going forever
                play_bg_music.run_if(run_once()),
            );
    }
}

#[derive(AssetCollection, Debug, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/Poppins-ExtraBoldItalic.ttf")]
    pub title: Handle<Font>,
}

#[derive(AssetCollection, Debug, Resource)]
pub struct SceneAssets {
    #[asset(path = "scenes/cheese_good.glb")]
    pub cheese_good: Handle<Scene>,
    #[asset(path = "scenes/cheese_ok.glb")]
    pub cheese_ok: Handle<Scene>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/Dirt_Weeds01.jpg")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/Dirt_Weeds01_disp.jpg")]
    pub ground_displacement: Handle<Image>,
    #[asset(path = "textures/Dirt_Weeds01_normal.jpg")]
    pub ground_normal: Handle<Image>,
    #[asset(path = "textures/bricks.png")]
    pub bricks: Handle<Image>,
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
    #[cfg_attr(not(target_arch = "wasm32"), asset(path = "audio/CheeseOnTheMoon.wav"))]
    #[cfg_attr(target_arch = "wasm32", asset(path = "audio/CheeseOnTheMoon.mp3"))]
    pub bg_track: Handle<AudioSource>,
}

fn play_bg_music(audio: Res<Audio>, bg_track: Res<AudioAssets>) {
    audio
        .play(bg_track.bg_track.clone())
        .looped()
        .with_volume(0.5);
}

#[derive(Component)]
struct LoadingUI;
#[derive(Component)]
struct LoadingUICamera;

fn spawn_loading_ui(mut commands: Commands) {
    commands.spawn((LoadingUICamera, Camera2dBundle::default()));
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
