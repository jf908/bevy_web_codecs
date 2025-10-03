# Bevy Web Codecs

[![Aiming.Pro](https://img.shields.io/badge/Aiming.Pro-open%20source-blue.svg?logo=data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMzIiIGhlaWdodD0iMzIiIHZpZXdCb3g9IjAgMCAzMiAzMiIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTE1LjQ4NzcgMjcuMjU3OEwxNiAyNC44NDgyTDE2LjUxMjIgMjcuMjU3OEMyMi4yMjI4IDI2Ljk4OCAyNi44MTQxIDIyLjMyMjkgMjcuMDc5NyAxNi41MjA1TDI0LjcwODIgMTZMMjcuMDc5NyAxNS40Nzk1QzI2LjgxNDEgOS42NzcwNyAyMi4yMjI4IDUuMDEyMDUgMTYuNTEyMiA0Ljc0MjE3TDE2IDcuMTUxOEwxNS40ODc3IDQuNzQyMTdDOS43NzcxMyA1LjAxMjA1IDUuMTg1ODYgOS42NzcwNyA0LjkyMDI1IDE1LjQ3OTVMNy4yOTE3NSAxNkw0LjkyMDI1IDE2LjUyMDVDNS4xODU4NiAyMi4zMjI5IDkuNzU4MTMgMjYuOTg4IDE1LjQ4NzcgMjcuMjU3OFpNMTcuMTU3MyAxLjczNDk0QzI0LjAwNjIgMi4yOTM5OCAyOS40ODkxIDcuODY1MDYgMzAuMDM5MyAxNC44MjQxTDMxLjc0NjggMTQuNDM4NVYxNlYxNy41NjE1TDMwLjAzOTMgMTcuMTc1OUMyOS40ODkxIDI0LjEzNDkgMjQuMDA2MiAyOS43MDYgMTcuMTU3MyAzMC4yNjUxTDE3LjUzNjcgMzJIMTZIMTQuNDYzMkwxNC44NDI3IDMwLjI2NTFDNy45OTM3MSAyOS43MDYgMi41MTA3OSAyNC4xMzQ5IDEuOTYwNjEgMTcuMTc1OUwwLjI1MzExMyAxNy41NjE1VjE2VjE0LjQzODVMMS45NjA2MSAxNC44MjQxQzIuNTEwNzkgNy44NjUwNiA3Ljk5MzcxIDIuMjkzOTggMTQuODQyNyAxLjczNDk0TDE0LjQ2MzIgMEgxNkgxNy41MzY3TDE3LjE1NzMgMS43MzQ5NFoiIGZpbGw9IiMxMDdGRUEiLz4KPHBhdGggZD0iTTE1Ljk2MjIgOC40ODIwNkwyMy4xMzM2IDIwLjk1NDRIMTkuNTY2OUwxNS45NjIyIDE0LjY4OTNMMTIuMzc2NSAyMC45NTQ0SDguNzkwNzdMMTUuOTYyMiA4LjQ4MjA2WiIgZmlsbD0id2hpdGUiLz4KPC9zdmc+Cg==)](https://aiming.pro)
[![crates.io](https://img.shields.io/crates/v/bevy_web_codecs.svg)](https://crates.io/crates/bevy_web_codecs)
[![docs.rs](https://docs.rs/bevy_web_codecs/badge.svg)](https://docs.rs/bevy_web_codecs/)
[![license](https://img.shields.io/crates/l/bevy_web_codecs)](https://github.com/jf908/bevy_web_codecs#license)

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
