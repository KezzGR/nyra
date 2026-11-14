use nyra::{App, Entity, EntityID, FreeFlyController, Material, Mesh, RunError};
use nyra_math::{Quat, Transform, Vec3};

struct Game {
    cube_a: EntityID,
    cube_b: EntityID,
    time: f32,
}

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup_with_state(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            let transform1 = Transform::new(Vec3::new(0.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
            let transform2 = Transform::new(Vec3::new(2.5, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
            let transform3 = Transform::new(Vec3::new(2.5, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);

            let material_red = ctx.assets.add_material(Material::new([0.9, 0.2, 0.2, 1.0]));
            let material_green = ctx.assets.add_material(Material::new([0.2, 0.9, 0.3, 1.0]));
            let material_blue = ctx.assets.add_material(Material::new([0.2, 0.4, 1.0, 1.0]));

            let cube_a = ctx
                .world
                .spawn(Entity::from_mesh(transform1, mesh).with_material(material_red));

            let cube_b = ctx
                .world
                .spawn(Entity::from_mesh(transform2, mesh).with_material(material_green));

            let cube_c = ctx
                .world
                .spawn(Entity::from_mesh(transform3, mesh).with_material(material_blue));

            ctx.world.set_parent(cube_c, cube_b);
            ctx.world.set_parent(cube_b, cube_a);

            Game {
                cube_a,
                cube_b,
                time: 0.0,
            }
        })
        .run_each_frame_with_state(move |ctx, state| {
            controller.update(ctx);
            state.time += ctx.delta_time;

            ctx.world
                .get_mut(state.cube_a)
                .unwrap()
                .transform
                .position
                .x += ctx.delta_time;

            ctx.world.get_mut(state.cube_b).unwrap().transform.rotation =
                Quat::from_rotation_y(state.time);
        })
}
