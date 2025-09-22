use bevy_app::{App, Plugin};
use bevy_asset::AssetApp;

mod image;

#[derive(Default)]
pub struct WebCodecsPlugin;

impl Plugin for WebCodecsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset_loader::<image::WebImageLoader>();
    }
}
