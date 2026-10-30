use nyra::{App, FreeFlyController, KeyCode, Material, Mesh, RunError, Thing, ThingID, Transform};
use nyra_math::{Quat, Vec3};

const GRID_DIM: u32 = 10;

fn main() -> Result<(), RunError> {
    let mut controller = FreeFlyController::default();

    App::new()
        .setup_with_state(|game| {
            let mesh_id = game.assets.add_mesh(Mesh::cube());

            let material_id = game.assets.add_material(Material::default());

            let mut cubes = Vec::default();

            for x in 0..GRID_DIM {
                for z in 0..GRID_DIM {
                    let transform = Transform::new(
                        Vec3::new(x as f32, 0.0, z as f32),
                        Quat::IDENTITY,
                        Vec3::splat(0.8),
                    );

                    let cube = game
                        .world
                        .spawn(Thing::new(transform, mesh_id).with_material(material_id));
                    cubes.push(cube);
                }
            }

            DeleteObjects {
                cubes,
                cube_id: 0,
                previous_cube: None,
            }
        })
        .run_each_frame_with_state(move |game, state| {
            controller.update(game);

            if game.input.key_just_pressed(KeyCode::Space) {
                if let Some(id) = state.cubes.pop() {
                    game.world.despawn(id);
                }
            }

            if game.input.key_just_pressed(KeyCode::Enter) {
                if let Some(&cube) = state.cubes.get(state.cube_id) {
                    if let Some(previous) = state.previous_cube {
                        game.world[previous].visible = true;
                    }

                    game.world[cube].visible = false;

                    state.previous_cube = Some(cube);
                    state.cube_id += 1;
                }
            }
        })
}

struct DeleteObjects {
    cubes: Vec<ThingID>,
    cube_id: usize,
    previous_cube: Option<ThingID>,
}
