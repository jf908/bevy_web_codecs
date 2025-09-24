# Bevy Web Codecs

This crate is a more efficient web replacement of Bevy's default image loaders by making use of the [WebCodecs API](https://developer.mozilla.org/en-US/docs/Web/API/WebCodecs_API) to decode images and fallsback to the canvas API when its unavailable. This crate parallelizes image decoding and reduces the bundle size compared to bevy_image's default decoders.

This crate is only supported on wasm targets.

## Usage

```toml
[dependencies]
bevy_web_codecs = "0.1"
```

```rust
use bevy::prelude::*;
use bevy_web_codecs::WebCodecsPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WebCodecsPlugin::default()))
        .run();
}
```

It's recommended that you turn off bevy's default features so you can disable "png" support so that it doesn't clash with this plugin's png loader.

### glTF Support

```toml
[dependencies]
bevy_web_codecs_gltf = { version = "0.16", features = ["bevy_animation"] }
```

```rust
use bevy::prelude::*;
use bevy_web_codecs::WebCodecsPlugin;
use bevy_web_codecs_gltf::GltfPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            WebCodecsPlugin::default(),
            GltfPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}
```

You will have to disable bevy's `bevy_gltf` feature and replace any imports:

```rust
// Before
use bevy::prelude::*;
use bevy::gltf::GltfExtras;

// After
use bevy::prelude::*;
use bevy_web_codecs_gltf::prelude::*;
use bevy_web_codecs_gltf::gltf::GltfExtras;
```

## Supported types

`WebCodecsPlugin` registers the following image types by default.

| Extension  | MIME Type  |
| ---------- | ---------- |
| .jpg, jpeg | image/jpeg |
| .png       | image/png  |
| .gif       | image/gif  |
| .webp      | image/webp |
| .bmp       | image/bmp  |
| .avif      | image/avif |

Registered file extensions and MIME types can be configured at startup but support will be limited
depending on your browser.

## Bevy version support

| bevy | bevy_web_codecs | bevy_web_codecs_gltf |
| ---- | --------------- | -------------------- |
| 0.16 | 0.1             | 0.16.1               |

## License

`bevy_web_codecs` is dual-licensed under either

- MIT License (./LICENSE-MIT or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 (./LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option.
