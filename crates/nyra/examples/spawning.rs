use nyra::{App, Entity, EntityID, KeyCode, Mesh, MeshID, RunError};
use nyra_math::{Quat, Transform, Vec3};

struct SpawnState {
    mesh: MeshID,
    objects: Vec<EntityID>,
}

fn main() -> Result<(), RunError> {
    println!("Space: spawn | Backspace: delete");

    App::new()
        .setup_with_state(|ctx| SpawnState {
            mesh: ctx.assets.add_mesh(Mesh::cube()),
            objects: Vec::new(),
        })
        .run_each_frame_with_state(|ctx, state| {
            if ctx.input.key_just_pressed(KeyCode::Space) {
                let index = state.objects.len();

                let x = (index % 7) as f32 * 1.3 - 3.9;
                let z = (index / 7) as f32 * 1.5;

                let transform = Transform::new(
                    Vec3::new(x, 0.0, z),
                    Quat::from_rotation_y(-0.6),
                    Vec3::splat(0.9),
                );

                let id = ctx.world.spawn(Entity::from_mesh(transform, state.mesh));

                state.objects.push(id);
            }

            if ctx.input.key_just_pressed(KeyCode::Backspace) {
                if let Some(id) = state.objects.pop() {
                    ctx.world.despawn(id);
                }
            }
        })
}
