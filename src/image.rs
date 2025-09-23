use anyhow::Result;
use bevy_asset::{AssetLoader, LoadContext, RenderAssetUsages, io::Reader};
use bevy_image::{Image, ImageSampler, TextureError};
use bevy_platform::collections::HashMap;
use image::DynamicImage;
use js_sys::Error;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/image.js")]
extern "C" {
    type BevyImageDecoder;

    #[wasm_bindgen(constructor)]
    fn new() -> BevyImageDecoder;

    #[wasm_bindgen(static_method_of = BevyImageDecoder)]
    fn supportsImageDecoder() -> bool;

    #[wasm_bindgen(catch, method)]
    async fn decode(this: &BevyImageDecoder, data: &[u8], mime_type: &str) -> Result<(), Error>;
    #[wasm_bindgen(method)]
    fn width(this: &BevyImageDecoder) -> u32;
    #[wasm_bindgen(method)]
    fn height(this: &BevyImageDecoder) -> u32;

    #[wasm_bindgen(catch, method)]
    async fn copy(this: &BevyImageDecoder, buffer: &mut [u8]) -> Result<(), Error>;
}

/// Possible errors that can be produced by [`OggLoader`]
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum WebImageLoaderError {
    /// An [IO Error](std::io::Error)
    #[error("Could not read the file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Could not load texture file: {0}")]
    FileTexture(#[from] FileTextureError),
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub enum ImageFormatSetting {
    #[default]
    FromExtension,
    MimeType(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImageLoaderSettings {
    pub format: ImageFormatSetting,
    pub sampler: ImageSampler,
    pub asset_usage: RenderAssetUsages,
}

impl Default for ImageLoaderSettings {
    fn default() -> Self {
        Self {
            format: ImageFormatSetting::default(),
            sampler: ImageSampler::Default,
            asset_usage: RenderAssetUsages::default(),
        }
    }
}

/// Asset loader for Jpeg files.
pub struct WebImageLoader {
    mime_types: HashMap<&'static str, &'static str>,
    extensions: Vec<&'static str>,
}

impl WebImageLoader {
    pub fn new(mime_types: HashMap<&'static str, &'static str>) -> Self {
        let extensions = mime_types.keys().copied().collect();

        Self {
            mime_types,
            extensions,
        }
    }

    pub fn supported_mime_types() -> HashMap<&'static str, &'static str> {
        let mut mime_types = HashMap::new();
        mime_types.insert("jpg", "image/jpeg");
        mime_types.insert("jpeg", "image/jpeg");
        mime_types.insert("png", "image/png");
        mime_types.insert("gif", "image/gif");
        mime_types.insert("webp", "image/webp");
        mime_types.insert("svg", "image/svg+xml");
        mime_types.insert("bmp", "image/bmp");
        mime_types.insert("avif", "image/avif");

        mime_types
    }

    pub fn supports_image_decoder() -> bool {
        BevyImageDecoder::supportsImageDecoder()
    }
}

impl Default for WebImageLoader {
    fn default() -> Self {
        Self::new(Self::supported_mime_types())
    }
}

impl AssetLoader for WebImageLoader {
    type Asset = Image;
    type Settings = ImageLoaderSettings;
    type Error = WebImageLoaderError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        settings: &ImageLoaderSettings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = vec![];
        reader.read_to_end(&mut bytes).await?;

        let mime_type: &str = match &settings.format {
            ImageFormatSetting::FromExtension => {
                let ext = load_context.path().extension().unwrap().to_str().unwrap();
                self.mime_types.get(ext).ok_or_else(|| FileTextureError {
                    error: TextureError::InvalidImageExtension(format!("{ext:?}")),
                    path: format!("{}", load_context.path().display()),
                })?
            }
            ImageFormatSetting::MimeType(format) => format,
        };

        let path = load_context.path();

        let decoder = BevyImageDecoder::new();
        decoder
            .decode(&bytes, mime_type)
            .await
            .map_err(|err| FileTextureError {
                error: TextureError::TranscodeError(err.to_string().into()),
                path: format!("{}", path.display()),
            })?;

        let width = decoder.width();
        let height = decoder.height();

        let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];

        decoder
            .copy(&mut buffer)
            .await
            .map_err(|err| FileTextureError {
                error: TextureError::TranscodeError(err.to_string().into()),
                path: format!("{}", path.display()),
            })?;

        let dyn_img = DynamicImage::ImageRgba8(
            image::RgbaImage::from_raw(width, height, buffer)
                .expect("Invalid image size when creating RgbaImage"),
        );

        Ok(Image::from_dynamic(dyn_img, true, settings.asset_usage))
    }

    fn extensions(&self) -> &[&str] {
        &self.extensions
    }
}

#[derive(Error, Debug)]
#[error("Error reading image file {path}: {error}")]
pub struct FileTextureError {
    error: TextureError,
    path: String,
}
