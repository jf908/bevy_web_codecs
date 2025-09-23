use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;
use bevy_platform::collections::HashMap;

use crate::image::WebImageLoader;

pub mod image;

pub struct WebCodecsPlugin {
    pub image_types: HashMap<&'static str, &'static str>,
}

impl Default for WebCodecsPlugin {
    fn default() -> Self {
        Self {
            image_types: WebImageLoader::supported_mime_types(),
        }
    }
}

impl Plugin for WebCodecsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(WebImageLoader::new(self.image_types.clone()));

        if !WebImageLoader::supports_image_decoder() {
            bevy_log::warn!(
                "ImageDecoder is not supported in this browser, WebCodecsPlugin is falling back to canvas decoding."
            );
        }
    }
}
