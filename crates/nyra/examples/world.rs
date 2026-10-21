use glam::{Quat, Vec3};
use nyra::{App, FreeFlyController, Mesh, RunError, Thing, Transform};

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup(|game| {
            let mesh_id = game.world.add_mesh(Mesh::cube());

            let left_cube_transform = Transform::new(Vec3::X * -3.0, Quat::IDENTITY, Vec3::ONE);
            let right_cube_transform = Transform::new(Vec3::X * 3.0, Quat::IDENTITY, Vec3::ONE);
            let idk_cube_transform =
                Transform::new(Vec3::new(-5.0, -3.0, 2.0), Quat::IDENTITY, Vec3::ONE);

            let left_cube = Thing::new(left_cube_transform, mesh_id);
            let right_cube = Thing::new(right_cube_transform, mesh_id);
            let idk_cube = Thing::new(idk_cube_transform, mesh_id);

            game.world.spawn(left_cube);
            game.world.spawn(right_cube);
            game.world.spawn(idk_cube);
        })
        .run(move |game| {
            controller.update(game);
        })
}
