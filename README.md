# Nyra 🦀

Nyra is a small, experimental 3D game engine I'm building in Rust.

I started it because I wanted to understand how game engines work under the hood, not just learn how to use one. It began with a triangle, then a cube, and somehow turned into a renderer, a world system, and a math library of my own.

It's still early, and the API changes whenever I find a better way to do something.

![100 cubes moving in a wave](assets/cube-wave.gif)

*100 animated cubes sharing one mesh. No instancing yet.*

## Getting started

```bash
git clone https://github.com/KezzGR/nyra.git
cd nyra
cargo run -p nyra --example textures
```

Other examples:

| Example | What it shows |
| --- | --- |
| `world` | Creating, moving, and accessing objects |
| `cube_wave` | Animating 100 objects that share a mesh and material |
| `delete_objects` | Removing objects and changing their visibility |
| `materials` | Different colors and shared material resources |
| `textures` | Loading a PNG texture and reusing it across materials |
| `input_test` | Basic keyboard input |

Run any of them with `cargo run -p nyra --example <name>`.

In examples with the free-fly camera, use **WASD** to move and the **mouse** to look around. Press **Escape** to release the cursor, then click the window to capture it again.

## What works

- **Rendering:** 3D meshes through `wgpu`, perspective projection, depth testing, and basic directional lighting.
- **Procedural geometry:** cubes and cuboids, with meshes that can be shared by multiple objects. Newly added meshes can be uploaded while the application runs.
- **Assets:** meshes, materials, and textures are stored separately from world objects in `Assets` and referenced by `MeshID`, `MaterialID`, and `TextureID`.
- **Materials and textures:** RGBA base colors, PNG texture loading, textured materials, and reuse of the same texture across multiple materials. Material colors can be updated at runtime. Transparency is not implemented yet.
- **World:** spawning, accessing, moving, hiding, and despawning objects using `ThingID`.
- **Application and input:** a managed `winit` event loop, keyboard and mouse input, delta time, cursor control, window focus checks, and a free-fly camera.
- **Math:** Nyra uses its own `nyra_math` crate for vectors, quaternions, matrices, transforms, and camera calculations instead of `glam`.

## A small example

This creates a cube with a material and moves it upward every frame:

```rust
use nyra::{App, Material, Mesh, RunError, Thing, ThingID, Transform};

struct GameState {
    cube: ThingID,
}

fn main() -> Result<(), RunError> {
    App::<GameState>::new()
        .setup(|game| {
            let mesh = game.assets.add_mesh(Mesh::cube());
            let material = game.assets.add_material(Material::default());

            let cube = game.world.spawn(
                Thing::new(Transform::default(), mesh)
                    .with_material(material),
            );

            GameState { cube }
        })
        .run_with_state(|game, state| {
            game.world[state.cube].transform.position.y += game.delta_time;
        })
}
```

`setup()` runs once and returns the application's state. `run_with_state()` receives that state every frame. For applications that don't need a separate state, there's also `App::run()`.

A `Thing` without a material is currently skipped by the mesh render pipeline. `Material::default()` is an ordinary material and must be assigned explicitly.

See [the examples](crates/nyra/examples) for more.

## Project structure

This workspace currently contains two crates:

- **[`nyra`](crates/nyra)**: rendering, application lifecycle, world, asset storage, camera, input, and GPU resource management.
- **[`nyra_math`](crates/nyra_math)**: my own math implementations, including `Vec2`, `Vec3`, `Quat`, `Mat4`, and view/projection calculations, with tests for mathematical operations.

## What's missing

There are no imported 3D models, physically based materials, shadows, physics, skeletal animation, or editor yet. Lighting is still hard-coded in the shader, and the renderer uses one basic mesh pipeline.

New meshes and textures can be uploaded at runtime, but editing an already uploaded mesh or texture doesn't currently refresh its GPU copy. There's no instanced rendering yet, and performance hasn't been systematically benchmarked.

## Built with

Rust · [`wgpu`](https://github.com/gfx-rs/wgpu) · [`winit`](https://github.com/rust-windowing/winit) · [`image`](https://github.com/image-rs/image) · `bytemuck` · `pollster` · `nyra_math`

## License

MIT.
