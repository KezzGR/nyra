use nyra::{App, Material, Mesh, RunError, Texture, Thing, Transform};
use nyra_math::{Quat, Vec3};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cube());

            let texture = ctx.assets.add_texture(Texture::from_file("assets/cat.png"));

            let original = ctx
                .assets
                .add_material(Material::new([1.0; 4]).with_texture(texture));

            let tinted = ctx
                .assets
                .add_material(Material::new([1.0, 0.5, 0.7, 1.0]).with_texture(texture));

            for (x, material) in [(-1.3, original), (1.3, tinted)] {
                let transform = Transform::new(
                    Vec3::new(x, 0.0, 0.0),
                    Quat::from_rotation_y(-0.35),
                    Vec3::ONE,
                );

                ctx.world
                    .spawn(Thing::new(transform, mesh).with_material(material));
            }
        })
        .run()
}
