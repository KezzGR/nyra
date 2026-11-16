use nyra::{App, Entity, Mesh, RunError};
use nyra_math::{Quat, Transform, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let low = ctx.assets.add_mesh(Mesh::uv_sphere(0.5, 8, 4));

            let smooth = ctx.assets.add_mesh(Mesh::sphere());

            for (x, mesh) in [(-1.0, low), (1.0, smooth)] {
                let transform = Transform::new(Vec3::new(x, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
                ctx.world.spawn(Entity::from_mesh(transform, mesh));
            }
        })
        .run()
}
