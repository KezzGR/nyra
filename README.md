# Nyra 🦀

I'm building my own 3D game engine in Rust.

Why? Mostly because I wanted to understand how game engines actually work instead of just using one.

So I started writing my own renderer, input system, camera, and everything else, one piece at a time.

The cube has friends now. And they move.

![Nyra cube wave demo](assets/cube-wave.gif)

*100 cubes, one shared mesh, and a sine wave. No animation system required.*

## Getting Started

Clone the repository and run the cube wave example:

```bash
git clone https://github.com/KezzGR/nyra.git
cd nyra
cargo run -p nyra --example cube_wave
```

Or try the other examples:

```bash
cargo run -p nyra --example world
cargo run -p nyra --example camera_move
```

**Controls:**

- **WASD**: Move around
- **Mouse**: Look around
- **Escape**: Release the cursor
- **Left click**: Capture it again

## What Works So Far

### Rendering

- 3D rendering with `wgpu`
- Procedural cube and cuboid generation
- Indexed meshes with face normals
- Multiple objects sharing a single mesh
- Perspective projection and depth buffering
- Basic directional Lambert lighting
- Window resizing

### World & Objects

- `World` stores meshes and objects (`Thing`)
- `Transform` controls position, rotation, and scale
- `world.spawn()` returns a `ThingID`
- Objects can be accessed with `world[id]`
- Object transforms can be changed every frame

### Camera & Input

- Free-fly camera with mouse and keyboard controls
- Quaternion-based camera rotation
- World-space and local-space movement
- Pressed, held, and released keyboard states
- Mouse movement tracking
- Delta time for frame-independent movement
- Cursor locking and visibility controls

### Application

Nyra handles the window, event loop, and rendering internally. Game code runs through `App::setup()` and `App::run()` without dealing with `winit` directly.

## The API

Here's a small example that creates a cube and moves it upward every frame:

```rust
use nyra::{App, Mesh, RunError, Thing, Transform};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|game| {
            let mesh_id = game.world.add_mesh(Mesh::cube());
            game.world.spawn(Thing::new(Transform::default(), mesh_id));
        })
        .run(|game| {
            game.world.things[0].transform.position.y += game.delta_time;
        })
}
```

For a more interesting example, see [`cube_wave.rs`](crates/nyra/examples/cube_wave.rs). It animates a grid of 100 cubes using a sine wave, without touching the renderer.

## Project Structure

Nyra currently consists of two Rust crates:

- **`nyra`**: Rendering, application lifecycle, world, camera, input, and other engine systems.
- **`nyra_math`**: My own math experiments, currently including a custom `Vec3` implementation.

The engine also uses `glam` for camera and transform math.

## Current State

This is still a very early project. There's a basic world and object API, but no full scene system, physics, asset loading, textures, or materials yet.

New meshes are currently uploaded to the GPU after `setup()`, not dynamically during the game loop. Rendering is also not instanced yet.

I'm learning Rust and graphics programming while building this, so expect the code and API to change quite a bit.

For now, I'm just building things, breaking things, figuring out why they broke, and moving on to the next piece.

## Built With

- Rust
- `wgpu`
- `winit`
- `glam`
- `bytemuck`
- `pollster`

## License

MIT
