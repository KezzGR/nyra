use glam::{Quat, Vec3};
use nyra::{App, FreeFlyController, Mesh, RunError, Thing, ThingID, Transform};
use winit::keyboard::KeyCode;

const GRID_DIM: u32 = 10;

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::<DeleteObjects>::new()
        .setup(|game| {
            let mesh_id = game.world.add_mesh(Mesh::cube());

            let mut cubes = Vec::default();

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let transform = Transform::new(
                        Vec3::new(x as f32, 0.0, z as f32),
                        Quat::IDENTITY,
                        Vec3::splat(0.8),
                    );

                    let cube = game.world.spawn(Thing::new(transform, mesh_id));
                    cubes.push(cube);
                }
            }

            DeleteObjects { cubes, cube_id: 0 }
        })
        .run_with_state(move |game, state| {
            controller.update(game);

            if game.input.key_just_pressed(KeyCode::Space) {
                if let Some(&id) = state.cubes.get(state.cube_id) {
                    game.world.despawn(id);
                    state.cube_id += 1;
                }
            }
        })
}

struct DeleteObjects {
    cubes: Vec<ThingID>,
    cube_id: usize,
}
