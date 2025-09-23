use bevy::prelude::*;
use bevy_web_codecs::WebCodecsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WebCodecsPlugin::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    let image_node_size = Node {
        width: Val::Px(240.0),
        height: Val::Px(160.0),
        ..default()
    };

    commands.spawn((
        Node {
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            display: Display::Flex,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(4.0),
            column_gap: Val::Px(4.0),
            ..default()
        },
        children![
            (
                image_node_size.clone(),
                ImageNode {
                    image: asset_server.load("jpeg.jpg"),
                    ..default()
                }
            ),
            (
                image_node_size.clone(),
                ImageNode {
                    image: asset_server.load("png.png"),
                    ..default()
                }
            ),
            (
                image_node_size.clone(),
                ImageNode {
                    image: asset_server.load("webp.webp"),
                    ..default()
                }
            ),
            (
                image_node_size.clone(),
                ImageNode {
                    image: asset_server.load("gif.gif"),
                    ..default()
                }
            ),
            (
                image_node_size.clone(),
                ImageNode {
                    image: asset_server.load("avif.avif"),
                    ..default()
                }
            ),
        ],
    ));
}
