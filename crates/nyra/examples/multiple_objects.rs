use nyra::{App, Entity, Mesh, RunError};
use nyra_math::{Quat, Transform, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            for x in [-2.2, 0.0, 2.2] {
                let transform = Transform::new(
                    Vec3::new(x, 0.0, 0.0),
                    Quat::from_rotation_y(-0.6),
                    Vec3::ONE,
                );

                ctx.world.spawn(Entity::from_mesh(transform, mesh));
            }
        })
        .run()
}
