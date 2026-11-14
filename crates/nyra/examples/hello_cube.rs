use nyra::{App, Entity, Mesh, RunError};
use nyra_math::{Quat, Transform};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            let mut cube = Entity::from_mesh(Transform::default(), mesh);

            cube.transform.rotation = Quat::from_rotation_y(-0.6);

            ctx.world.spawn(cube);
        })
        .run()
}
