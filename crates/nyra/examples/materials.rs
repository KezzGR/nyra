use nyra::{App, Material, Mesh, RunError, Thing, Transform};
use nyra_math::{Quat, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            let colors = [
                [0.95, 0.30, 0.40, 1.0],
                [0.35, 0.85, 0.55, 1.0],
                [0.35, 0.55, 0.95, 1.0],
            ];

            for (index, color) in colors.into_iter().enumerate() {
                let material = ctx.assets.add_material(Material::new(color));

                let transform = Transform::new(
                    Vec3::new(2.2 * (index as f32 - 1.0), 0.0, 0.0),
                    Quat::from_rotation_y(-0.6),
                    Vec3::ONE,
                );

                ctx.world
                    .spawn(Thing::new(transform, mesh).with_material(material));
            }
        })
        .run()
}
