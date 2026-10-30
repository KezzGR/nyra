use nyra::{App, Material, Mesh, RunError, Thing, Transform};
use nyra_math::Quat;

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());
            let material = ctx.assets.add_material(Material::default());

            let mut cube = Thing::new(Transform::default(), mesh).with_material(material);

            cube.transform.rotation = Quat::from_rotation_y(-0.6);

            ctx.world.spawn(cube);
        })
        .run()
}
