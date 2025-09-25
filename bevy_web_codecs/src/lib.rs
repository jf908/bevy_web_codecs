//! This crate is a more efficient web replacement of Bevy's default image loaders by making use of
//! the WebCodecs API to decode images and fallsback to the canvas API when its unavailable.
//! This crate parallelizes image decoding and reduces the bundle size compared to bevy_image's
//! default decoders.
//!
//! ## Example
//!
//! ```no_run
//! use bevy::prelude::*;
//! use bevy_web_codecs::WebCodecsPlugin;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, WebCodecsPlugin::default()))
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!     commands.spawn(Camera2d);
//!
//!     commands.spawn(Sprite::from_image(asset_server.load("my_image.png")));
//! }
//! ```

use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;
use bevy_platform::collections::HashMap;

use crate::image::WebImageLoader;

/// Image loading and decoding
pub mod image;

/// Plugin that initiates the WebImageLoader.
///
/// A warning will be printed at startup if the browser does not support the ImageDecoder API.
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
