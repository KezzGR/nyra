use nyra::{App, FreeFlyController, Material, Mesh, RunError, Thing, ThingID, Transform};
use nyra_math::{Quat, Vec3};

const GRID_DIM: u32 = 10;

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup_with_state(|game| {
            let center = (GRID_DIM as f32 - 1.0) / 2.0;

            let mesh_id = game.assets.add_mesh(Mesh::cube());

            let material_id = game.assets.add_material(Material::default());

            let mut cubes = Vec::default();

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let transform = Transform::new(
                        Vec3::new(x as f32 - center, 0.0, z as f32),
                        Quat::IDENTITY,
                        Vec3::splat(0.8),
                    );

                    let cube = game
                        .world
                        .spawn(Thing::new(transform, mesh_id).with_material(material_id));

                    cubes.push(cube);
                }
            }

            CubeWave {
                cubes,
                time: 0.0,
                center,
            }
        })
        .run_each_frame_with_state(move |game, state| {
            controller.update(game);

            let mut id: usize = 0;

            state.time += game.delta_time;

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

struct CubeWave {
    cubes: Vec<ThingID>,
    time: f32,
    center: f32,
}
