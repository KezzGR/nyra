use nyra::{App, FreeFlyController, Material, Mesh, RunError, Thing, Transform};
use nyra_math::{Quat, Vec3};

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            for (x, z) in [(-2.0, 0.0), (0.0, 0.0), (2.0, 0.0), (0.0, 3.0)] {
                let transform =
                    Transform::new(Vec3::new(x, 0.0, z), Quat::from_rotation_y(-0.6), Vec3::ONE);

                ctx.world
                    .spawn(Thing::new(transform, mesh).with_material(material));
            }
        })
        .run_each_frame(move |ctx| {
            controller.update(ctx);
        })
}
