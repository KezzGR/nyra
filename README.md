# Nyra 🦀

Nyra is a small, experimental 3D game engine I'm building in Rust.

I started it because I wanted to understand how game engines work under the hood, not just learn how to use one. It began with a triangle, then a cube, and somehow turned into a renderer, a world system, and a math library of my own.

It's still early, and the API changes whenever I find a better way to do something.

![100 cubes moving in a wave](assets/cube-wave.gif)

*100 animated cubes sharing one mesh and one material. No instancing yet.*

## Getting started

Clone the repository and run your first cube:

```bash
git clone https://github.com/KezzGR/nyra.git
cd nyra
cargo run -p nyra --example hello_cube
```

To explore the other examples, replace `hello_cube` with any name below.

## Examples

| Example | What it shows |
| --- | --- |
| [`hello_window`](crates/nyra/examples/hello_window.rs) | Open an empty window with `App::new().run()` |
| [`hello_cube`](crates/nyra/examples/hello_cube.rs) | Create a mesh, material, and your first 3D object |
| [`multiple_objects`](crates/nyra/examples/multiple_objects.rs) | Share the same mesh and material between multiple objects |
| [`transforms`](crates/nyra/examples/transforms.rs) | Position, rotate, and scale objects |
| [`materials`](crates/nyra/examples/materials.rs) | Give objects different colors using materials |
| [`textures`](crates/nyra/examples/textures.rs) | Load a PNG and reuse it with different material colors |
| [`keyboard_input`](crates/nyra/examples/keyboard_input.rs) | Respond to key presses and releases |
| [`free_camera`](crates/nyra/examples/free_camera.rs) | Move around a scene with the free-fly camera |
| [`animation`](crates/nyra/examples/animation.rs) | Update an object every frame using application state and delta time |
| [`spawning`](crates/nyra/examples/spawning.rs) | Spawn objects with Space and remove them with Backspace |
| [`cube_wave`](crates/nyra/examples/cube_wave.rs) | Animate 100 cubes sharing a mesh and material |

Run any example from the repository root:

```bash
cargo run -p nyra --example cube_wave
```

In examples with the free-fly camera, use **WASD** to move and the **mouse** to look around. Press **Escape** to release the cursor; click the window to capture it again.

## A small example

This creates a cube and rotates it every frame. `setup_with_state()` returns user-defined state, and `run_each_frame_with_state()` receives it on each frame:

```rust
use nyra::{App, Material, Mesh, RunError, Thing, ThingID, Transform};
use nyra_math::Quat;

struct AnimationState {
    cube: ThingID,
    angle: f32,
}

fn main() -> Result<(), RunError> {
    App::new()
        .setup_with_state(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            let cube = ctx.world.spawn(
                Thing::new(Transform::default(), mesh).with_material(material)
            );

            AnimationState { cube, angle: 0.0 }
        })
        .run_each_frame_with_state(|ctx, state| {
            state.angle += ctx.delta_time;
            ctx.world[state.cube].transform.rotation = Quat::from_rotation_y(state.angle);
        })
}
```

You don't have to use application state. For simpler programs, use `setup()` to initialize a scene and `run()` to display it, or `run_each_frame()` to execute a callback every frame without a separate state.

A `Thing` without a material is currently skipped by the mesh render pipeline. Assign a material explicitly, including when using `Material::default()`.

## What works

- **Rendering:** 3D meshes through `wgpu`, perspective projection, depth testing, and basic directional lighting.
- **Geometry:** built-in cubes and cuboids, user-supplied meshes, and uploading newly added meshes while the application runs.
- **Assets:** separate storage for meshes, materials, and textures, referenced by `MeshID`, `MaterialID`, and `TextureID`.
- **Materials and textures:** RGBA base colors, PNG loading, textured materials, reuse of textures across materials, and runtime material color changes. Transparency is not implemented yet.
- **World:** spawn, access, move, hide, and despawn objects using `ThingID`.
- **Application and input:** `winit` event loop, keyboard and mouse input, delta time, cursor control, and a free-fly camera.
- **Math:** a separate `nyra_math` crate with vectors, quaternions, matrices, transforms, and camera calculations instead of `glam`.

## Project structure

The workspace contains two crates:

- **[`nyra`](crates/nyra)**: rendering, application lifecycle, world, assets, camera, input, and GPU resource management.
- **[`nyra_math`](crates/nyra_math)**: my own implementations of `Vec2`, `Vec3`, `Quat`, `Mat4`, and view/projection math, with tests.

## What's missing

There is no 3D model importer, PBR, shadows, physics, skeletal animation, or editor yet. Lighting is still hard-coded in the shader, and rendering uses one basic mesh pipeline.

New meshes and textures can be uploaded at runtime, but modifying an already uploaded mesh or texture does not automatically update its GPU copy. There is no instanced rendering yet, and performance has not been systematically benchmarked.

## Built with

Rust · [`wgpu`](https://github.com/gfx-rs/wgpu) · [`winit`](https://github.com/rust-windowing/winit) · [`image`](https://github.com/image-rs/image) · `bytemuck` · `pollster` · `nyra_math`

## License

MIT.
