use nyra::{App, Material, MaterialID, Mesh, RunError, Texture, Thing, Transform};
use nyra_math::{Quat, Vec2, Vec3};

struct State {
    material: MaterialID,
}

fn main() -> Result<(), RunError> {
    App::new()
        .setup_with_state(|ctx| {
            let mesh = ctx.assets.add_mesh(Mesh::cuboid(Vec3::new(2.0, 4.0, 1.0)));

            let texture = ctx.assets.add_texture(Texture::from_file("assets/cat.png"));

            let original = ctx.assets.add_material(
                Material::new([1.0; 4])
                    .with_texture(texture)
                    .with_uv_scale(Vec2::new(3.0, 2.0)),
            );

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

            State { material: original }
        })
        .run_each_frame_with_state(|ctx, state| {
            let material = ctx
                .assets
                .material_storage
                .get_mut(&state.material)
                .unwrap();

            material.uv_offset.x += ctx.delta_time;
        })
}
