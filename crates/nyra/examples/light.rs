use nyra::{App, FreeFlyController, Material, Mesh, RunError, Thing, ThingID, Transform};
use nyra_math::{Quat, Vec3};

const GRID_DIM: u32 = 10;
const SUN_SPEED: f32 = 0.5;

struct LightDemo {
    cubes: Vec<ThingID>,
    time: f32,
    center: f32,
}

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup_with_state(|ctx| {
            let center = (GRID_DIM as f32 - 1.0) / 2.0;

            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            let mut cubes = Vec::new();

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let transform = Transform::new(
                        Vec3::new(x as f32 - center, 0.0, z as f32),
                        Quat::IDENTITY,
                        Vec3::splat(0.8),
                    );

                    let cube = ctx
                        .world
                        .spawn(Thing::new(transform, mesh).with_material(material));

                    cubes.push(cube);
                }
            }

            ctx.world.environment.sun.direction = Vec3::new(1.0, 1.5, 0.0);

            LightDemo {
                cubes,
                time: 0.0,
                center,
            }
        })
        .run_each_frame_with_state(move |game, state| {
            controller.update(game);

            state.time += game.delta_time;

            // Rotate the sun around the Y axis.
            let angle = state.time * SUN_SPEED;

            game.world.environment.sun.direction = Vec3::new(angle.cos(), 1.5, angle.sin());

            // Animate the cube wave.
            let mut id = 0;

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let dx = x as f32 - state.center;
                    let dz = z as f32 - state.center;

                    let distance = (dx * dx + dz * dz).sqrt();

                    game.world[state.cubes[id]].transform.position.y =
                        0.9 * (1.2 * distance - 3.0 * state.time).sin();

                    id += 1;
                }
            }
        })
}
