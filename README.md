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
cargo run -p nyra --example materials
```

Other examples:

| Example | What it shows |
| --- | --- |
| `world` | Creating, moving, and accessing objects |
| `cube_wave` | Animating 100 objects that share a mesh and material |
| `delete_objects` | Removing objects and changing their visibility |
| `materials` | Different colors and shared material resources |
| `input_test` | Basic keyboard input |

Run any of them with `cargo run -p nyra --example <name>`.

In examples with the free-fly camera, use **WASD** to move and the **mouse** to look around. Press **Escape** to release the cursor, then click the window to capture it again.

## What works

- **Rendering:** 3D meshes through `wgpu`, perspective projection, depth testing, and basic directional lighting.
- **Procedural geometry:** cubes and cuboids, with meshes that can be shared by multiple objects.
- **Materials:** solid RGBA base colors, shared `MaterialID` references, and GPU material buffers that can be updated while the application runs. Transparency is not implemented yet.
- **World:** spawning, accessing, moving, hiding, and despawning objects using `ThingID`.
- **Application and input:** a managed `winit` event loop, keyboard and mouse input, delta time, and a free-fly camera.
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
            let mesh = game.world.add_mesh(Mesh::cube());
            let material = game.world.add_material(Material::default());

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

- **[`nyra`](crates/nyra)**: rendering, application lifecycle, world, camera, input, and GPU resource management.
- **[`nyra_math`](crates/nyra_math)**: my own math implementations, including `Vec2`, `Vec3`, `Quat`, `Mat4`, and view/projection calculations, with tests for mathematical operations.

## What's missing

Quite a lot. There are no textures, imported 3D models, physically based materials, shadows, physics, skeletal animation, or editor yet. Lighting is still hard-coded in the shader, and the renderer uses one basic mesh pipeline.

New meshes can be uploaded at runtime, but editing an already uploaded mesh doesn't currently update its GPU copy. There's no instanced rendering yet, and performance hasn't been systematically benchmarked.

For now I'm focused on learning, keeping the engine understandable, and building up the features needed to make a small game with it.

## Built with

Rust · [`wgpu`](https://github.com/gfx-rs/wgpu) · [`winit`](https://github.com/rust-windowing/winit) · `bytemuck` · `pollster` · `nyra_math`

## License

MIT. See [LICENSE](LICENSE).
