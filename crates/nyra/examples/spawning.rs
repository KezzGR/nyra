use nyra::{App, KeyCode, Material, MaterialID, Mesh, MeshID, RunError, Thing, ThingID, Transform};
use nyra_math::{Quat, Vec3};

struct SpawnState {
    mesh: MeshID,
    material: MaterialID,
    objects: Vec<ThingID>,
    next: usize,
}

fn main() -> Result<(), RunError> {
    println!("Space: spawn | Backspace: delete");

    App::new()
        .setup_with_state(|ctx| SpawnState {
            mesh: ctx.assets.add_mesh(Mesh::cube()),
            material: ctx.assets.add_material(Material::default()),
            objects: Vec::new(),
            next: 0,
        })
        .run_each_frame_with_state(|ctx, state| {
            if ctx.input.key_just_pressed(KeyCode::Space) {
                let x = (state.next % 7) as f32 * 1.3 - 3.9;
                let z = (state.next / 7) as f32 * 1.5;

                let transform = Transform::new(
                    Vec3::new(x, 0.0, z),
                    Quat::from_rotation_y(-0.6),
                    Vec3::splat(0.9),
                );

                let id = ctx
                    .world
                    .spawn(Thing::new(transform, state.mesh).with_material(state.material));

                state.objects.push(id);
                state.next += 1;
            }

            if ctx.input.key_just_pressed(KeyCode::Backspace) {
                if let Some(id) = state.objects.pop() {
                    ctx.world.despawn(id);
                }
            }
        })
}
