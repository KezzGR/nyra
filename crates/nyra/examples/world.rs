use nyra::{App, FreeFlyController, Material, Mesh, RunError, Thing, Transform};
use nyra_math::{Quat, Vec3};

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup(|game| {
            let mesh_id = game.world.add_mesh(Mesh::cube());

            let material_id = game.world.add_material(Material::default());

            let left_cube_transform = Transform::new(Vec3::X * -3.0, Quat::IDENTITY, Vec3::ONE);
            let right_cube_transform = Transform::new(Vec3::X * 3.0, Quat::IDENTITY, Vec3::ONE);
            let idk_cube_transform =
                Transform::new(Vec3::new(-5.0, -3.0, 2.0), Quat::IDENTITY, Vec3::ONE);

            let left_cube = Thing::new(left_cube_transform, mesh_id).with_material(material_id);
            let right_cube = Thing::new(right_cube_transform, mesh_id).with_material(material_id);
            let idk_cube = Thing::new(idk_cube_transform, mesh_id).with_material(material_id);

            let left_cube = game.world.spawn(left_cube);
            game.world.spawn(right_cube);
            game.world.spawn(idk_cube);

            game.world[left_cube]
                .transform
                .translate_local(Vec3::Y * 2.0);
        })
        .run(move |game| {
            controller.update(game);

            game.world.things[0].transform.position.x += game.delta_time * 2.0;
        })
}
