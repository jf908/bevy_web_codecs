use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
// use bevy::platform::collections::HashMap;
use bevy_asset::io::Reader;
use bevy_asset::{AssetLoader, LoadContext, RenderAssetUsages};
use bevy_image::{Image, ImageSampler, TextureError};
use image::DynamicImage;
use js_sys::Uint8Array;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use wasm_bindgen_futures::JsFuture;
use web_sys::{ImageDecodeResult, ImageDecoder, ImageDecoderInit, VideoFrameCopyToOptions};

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

impl Default for WebImageLoader {
    fn default() -> Self {
        let mut mime_types = HashMap::new();
        mime_types.insert("jpg", "image/jpeg");
        mime_types.insert("jpeg", "image/jpeg");
        mime_types.insert("png", "image/png");
        mime_types.insert("gif", "image/gif");
        mime_types.insert("webp", "image/webp");
        mime_types.insert("bmp", "image/bmp");
        mime_types.insert("avif", "image/avif");

        let extensions = mime_types.keys().copied().collect();

        Self {
            mime_types,
            extensions,
        }
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
        let dyn_img = decode_image(mime_type, path, &bytes).await?;

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

async fn decode_image(
    mime_type: &str,
    path: &Path,
    bytes: &[u8],
) -> Result<DynamicImage, WebImageLoaderError> {
    let array = Uint8Array::from(bytes);

    let supported = JsFuture::from(ImageDecoder::is_type_supported(mime_type))
        .await
        .map(|v| v.as_bool().unwrap_or(false))
        .unwrap_or(false);

    if !supported {
        return Err(WebImageLoaderError::FileTexture(FileTextureError {
            error: TextureError::InvalidImageMimeType(mime_type.to_string()),
            path: format!("{}", path.display()),
        }));
    }

    let image_decoder_init = ImageDecoderInit::new(array.as_ref(), mime_type);
    let image_decoder = ImageDecoder::new(&image_decoder_init).map_err(|err| FileTextureError {
        error: TextureError::TranscodeError(err.as_string().unwrap_or_default()),
        path: format!("{}", path.display()),
    })?;

    let decoded = JsFuture::from(image_decoder.decode())
        .await
        .expect("The ImageDecoder was closed before the image had finished decoding.");

    let frame = ImageDecodeResult::get_image(&ImageDecodeResult::from(decoded));

    let width = frame.coded_width();
    let height = frame.coded_height();

    let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];

    let options = VideoFrameCopyToOptions::new();
    options.set_format("RGBA");

    JsFuture::from(frame.copy_to_with_u8_slice_and_options(&mut buffer, &options))
        .await
        .map_err(|err| FileTextureError {
            error: TextureError::TranscodeError(err.as_string().unwrap_or_default()),
            path: format!("{}", path.display()),
        })?;

    Ok(DynamicImage::ImageRgba8(
        image::RgbaImage::from_raw(width, height, buffer)
            .expect("Invalid image size when creating RgbaImage"),
    ))
}
