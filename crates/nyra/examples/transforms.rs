use nyra::{App, Entity, Mesh, RunError};
use nyra_math::{Quat, Transform, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            let transforms = [
                Transform::new(
                    Vec3::new(-2.2, 0.0, 0.0),
                    Quat::from_rotation_y(-0.6),
                    Vec3::splat(0.7),
                ),
                Transform::new(Vec3::ZERO, Quat::from_rotation_y(-0.9), Vec3::ONE),
                Transform::new(
                    Vec3::new(2.2, 0.0, 0.0),
                    Quat::from_rotation_z(0.35) * Quat::from_rotation_y(-0.6),
                    Vec3::new(0.7, 1.5, 0.7),
                ),
            ];

            for transform in transforms {
                ctx.world.spawn(Entity::from_mesh(transform, mesh));
            }
        })
        .run()
}
