use nyra::{App, Material, Mesh, RunError, Thing, Transform};
use nyra_math::{Quat, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            for x in [-2.2, 0.0, 2.2] {
                let transform = Transform::new(
                    Vec3::new(x, 0.0, 0.0),
                    Quat::from_rotation_y(-0.6),
                    Vec3::ONE,
                );

                ctx.world
                    .spawn(Thing::new(transform, mesh).with_material(material));
            }
        })
        .run()
}
