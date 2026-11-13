use nyra::{App, Entity, EntityID, FreeFlyController, Material, Mesh, RunError};
use nyra_math::{Quat, Transform, Vec3};

struct Game {
    cube_a: EntityID,
    cube_b: EntityID,
    cube_c: EntityID,
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

            let material1 = ctx.assets.add_material(Material::new([0.9, 0.2, 0.2, 1.0])); // Красный
            let material2 = ctx.assets.add_material(Material::new([0.2, 0.9, 0.3, 1.0])); // Зелёный
            let material3 = ctx.assets.add_material(Material::new([0.2, 0.4, 1.0, 1.0])); // Синий

            let cube_a = ctx
                .world
                .spawn(Entity::new(transform1, mesh).with_material(material1));

            let cube_b = ctx
                .world
                .spawn(Entity::new(transform2, mesh).with_material(material2));

            let cube_c = ctx
                .world
                .spawn(Entity::new(transform3, mesh).with_material(material3));

            ctx.world.set_parent(cube_c, cube_b);
            ctx.world.set_parent(cube_b, cube_a);

            Game {
                cube_a,
                cube_b,
                cube_c,
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
