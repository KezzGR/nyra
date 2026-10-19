# Nyra

A lightweight 3D engine in Rust for anime-style games.

Nyra is in early development. It currently renders a procedurally generated spinning cube with perspective projection, basic Lambert lighting, and a depth buffer.

Depth is no longer a lie.

## Why

Nyra aims to bring anime-style character rendering tools together in a lightweight engine: toon shading, VRM, morph targets, and lip-sync.

These are long-term goals, not implemented features.

## Goals

- Built-in toon shading, VRM, morph targets, and lip-sync.
- Text-based scene format, mergeable in git.
- Lightweight editor that runs on weak hardware.
- Simple, stable API with room for low-level experimentation..

## Current Features

### App and Input

- Window creation and resize handling via `winit`.
- User-defined per-frame update callback via `App::new().run(|game| { ... })`.
- Read-only input access through `GameContext`.
- Keyboard state queries:
  - `key_down(key)`: true while a key is held.
  - `key_just_pressed(key)`: true for the update following a new press.
  - `key_just_released(key)`: true for the update following a release.
- Repeated key events do not count as additional presses.

### Rendering and Math

- Procedural cube and cuboid generation with configurable dimensions.
- Indexed cube geometry: 24 vertices with face normals, 36 indices.
- Custom `Vec3` type in `nyra_math`, used for procedural mesh generation.
- Position, quaternion rotation, and scale transforms.
- Basic fixed camera with view and perspective projection matrices.
- Model and projection-view matrices passed through a uniform buffer.
- Directional Lambert lighting in WGSL.
- Depth testing and depth writes using a `Depth32Float` buffer.
- Depth buffer recreated when the window is resized.

## Example: Keyboard Input

```rust
use nyra::App;
use winit::{error::EventLoopError, keyboard::KeyCode};

fn main() -> Result<(), EventLoopError> {
    App::new().run(|game| {
        if game.input.key_just_pressed(KeyCode::Space) {
            println!("Hello from Nyra!");
        }

        if game.input.key_down(KeyCode::KeyW) {
            // W is being held.
        }

        if game.input.key_just_released(KeyCode::KeyW) {
            println!("W released!");
        }
    })
}
```

The closure is called during the application's frame updates. Nyra handles the `winit` event loop and tracks keyboard state internally.

To try the existing example:

```sh
cargo run -p nyra --example input_test
```

## Stack

- Rust (edition 2024)
- `wgpu` — rendering
- `winit` — window and input
- `bytemuck` — CPU↔GPU data transfer
- `pollster` — sync/async bridge
- `glam` — transform and camera math
- `nyra_math` - custom vector math for procedural mesh generation

## License

MIT
