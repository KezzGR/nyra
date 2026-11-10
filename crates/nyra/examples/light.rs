use nyra::{App, Entity, FreeFlyController, Material, Mesh, RunError};
use nyra_math::{Quat, Transform, Vec3};

const SUN_SPEED: f32 = 0.5;
const SUN_HEIGHT: f32 = 1.5;

struct LightDemo {
    time: f32,
}

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup_with_state(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            for (x, rotation) in [(-2.2, -0.6), (0.0, 0.0), (2.2, 0.6)] {
                let transform = Transform::new(
                    Vec3::new(x, 0.0, 0.0),
                    Quat::from_rotation_y(rotation),
                    Vec3::ONE,
                );

                ctx.world
                    .spawn(Entity::new(transform, mesh).with_material(material));
            }

            ctx.world.environment.ambient_intensity = 0.2;
            ctx.world.environment.sun.intensity = 0.8;
            ctx.world.environment.sun.direction = Vec3::new(1.0, SUN_HEIGHT, 0.0);

            LightDemo { time: 0.0 }
        })
        .run_each_frame_with_state(move |ctx, state| {
            controller.update(ctx);

            state.time += ctx.delta_time;

            let angle = state.time * SUN_SPEED;

            ctx.world.environment.sun.direction = Vec3::new(angle.cos(), SUN_HEIGHT, angle.sin());
        })
}
