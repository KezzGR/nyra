use nyra::{App, Material, Mesh, RunError, Thing, Transform};

fn main() -> Result<(), RunError> {
    App::new()
        .setup(|game| {
            let cube = game.assets.add_mesh(Mesh::cube());
            let material = game.assets.add_material(Material::default());

            game.world
                .spawn(Thing::new(Transform::default(), cube).with_material(material));
        })
        .run()
}
