//! Test the WebCodecs version of GltfPlugin by loading a gltf file that contains jpegs without
//! bevy's jpeg feature enabled.

use bevy::prelude::*;
use bevy_web_codecs::WebCodecsPlugin;
use bevy_web_codecs_gltf::{GltfAssetLabel, GltfPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WebCodecsPlugin::default())
        .add_plugins(GltfPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

/// Creates the scene.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3d::default())
        .insert(Transform::from_xyz(-0.7, 0.7, 1.0).looking_at(vec3(0.0, 0.3, 0.0), Vec3::Y));

    commands.spawn(SceneRoot(asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("MixedLightingExample.gltf"),
    )));
}
