use nyra::{App, FreeFlyController, Material, Mesh, RunError, Texture, Thing, ThingID, Transform};
use nyra_math::{Quat, Vec3};

const PINK: [f32; 4] = [0.96, 0.36, 0.59, 1.0];
const BLUE: [f32; 4] = [0.36, 0.55, 0.95, 1.0];
const GREEN: [f32; 4] = [0.34, 0.79, 0.56, 1.0];

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::<MaterialsDemo>::new()
        .setup(|game| {
            let mesh = game.assets.add_mesh(Mesh::cube());

            let texture = Texture::from_file("assets/cat.png");
            let texture_id = game.assets.add_texture(texture);

            let pink = game
                .assets
                .add_material(Material::new(PINK).with_texture(texture_id));
            let blue = game
                .assets
                .add_material(Material::new(BLUE).with_texture(texture_id));
            let green = game
                .assets
                .add_material(Material::new(GREEN).with_texture(texture_id));

            let materials = [None, Some(pink), Some(blue), Some(green), Some(pink)];

            let cubes = std::array::from_fn(|i| {
                let x = (i as f32 - 2.0) * 1.7;

                let transform = Transform::new(Vec3::new(x, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);

                let mut thing = Thing::new(transform, mesh);

                if let Some(material) = materials[i] {
                    thing = thing.with_material(material);
                }

                game.world.spawn(thing)
            });

            MaterialsDemo { cubes, time: 0.0 }
        })
        .run_with_state(move |game, state| {
            controller.update(game);

            state.time += game.delta_time;

            for (i, id) in state.cubes.iter().enumerate() {
                let speed = 0.4 + i as f32 * 0.15;

                game.world[*id].transform.rotation = Quat::from_rotation_y(state.time * speed);
            }
        })
}

struct MaterialsDemo {
    cubes: [ThingID; 5],
    time: f32,
}
