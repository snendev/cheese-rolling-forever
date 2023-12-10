use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSampler, ImageSamplerDescriptor},
};
use bevy_asset_loader::prelude::*;

use crate::AppState;

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/Dirt_Weeds01.jpg")]
    pub ground: Handle<Image>,
    #[asset(path = "textures/Dirt_Weeds01_disp.jpg")]
    pub ground_displacement: Handle<Image>,
    #[asset(path = "textures/Dirt_Weeds01_normal.jpg")]
    pub ground_normal: Handle<Image>,
}

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
        .add_systems(OnExit(AppState::Loading), tile_terrain_assets);
    }
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
