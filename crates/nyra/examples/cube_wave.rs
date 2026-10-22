use glam::{Quat, Vec3};
use nyra::{App, FreeFlyController, Mesh, RunError, Thing, Transform};

const GRID_DIM: u32 = 10;

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    let mut time: f32 = 0.0;

    let center = (GRID_DIM as f32 - 1.0) / 2.0;

    App::new()
        .setup(|game| {
            let mesh_id = game.world.add_mesh(Mesh::cube());

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let transform = Transform::new(
                        Vec3::new(x as f32, 0.0, z as f32),
                        Quat::IDENTITY,
                        Vec3::splat(0.8),
                    );

                    game.world.spawn(Thing::new(transform, mesh_id));
                }
            }
        })
        .run(move |game| {
            controller.update(game);

            let mut id: usize = 0;

            time += game.delta_time;

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let dx = x as f32 - center;
                    let dz = z as f32 - center;

                    let distance = (dx * dx + dz * dz).sqrt();

                    game.world.things[id].transform.position.y =
                        0.9 * (1.2 * distance - 3.0 * time).sin();
                    id += 1;
                }
            }
        })
}
