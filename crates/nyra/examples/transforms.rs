use nyra::{App, Material, Mesh, RunError, Thing, Transform};
use nyra_math::{Quat, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

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
                ctx.world
                    .spawn(Thing::new(transform, mesh).with_material(material));
            }
        })
        .run()
}
